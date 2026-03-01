#!/usr/bin/env python3
"""
Hawkeye ingest pipeline benchmark — measures end-to-end throughput.

Triggers ingestion via the Hawkeye API and polls status until completion,
measuring wall-clock time, files/second, and success/failure counts.

Usage:
    # Benchmark with default settings (all test_data/, server on 7700):
    uv run scripts/benchmark_ingest.py

    # Benchmark only 20 files (sorted, same set every run):
    uv run scripts/benchmark_ingest.py --limit 20 --clean --output results_mlx.json

    # Clean DB first (for fair A/B comparison between sidecars):
    uv run scripts/benchmark_ingest.py --clean

    # Save results to JSON for later comparison:
    uv run scripts/benchmark_ingest.py --output results_mlx.json

    # Compare two saved results:
    uv run scripts/benchmark_ingest.py --compare results_mlx.json results_vllm.json
"""
# /// script
# requires-python = ">=3.11"
# dependencies = ["httpx", "rich"]
# ///

import argparse
import atexit
import json
import os
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

import httpx
from rich.console import Console
from rich.table import Table
from rich import box
from rich.live import Live
from rich.text import Text

HAWKEYE_URL = "http://localhost:7700"
POSTGRES_URL = "postgresql://hawkeye:hawkeye@localhost:5433/hawkeye"
POLL_INTERVAL = 2.0


def check_server(url: str, console: Console) -> bool:
    try:
        resp = httpx.get(f"{url}/status", timeout=3)
        return resp.status_code == 200
    except Exception:
        return False


def check_sidecar(url: str) -> dict:
    try:
        resp = httpx.get(f"{url}/v1/models", timeout=3)
        if resp.status_code == 200:
            data = resp.json()
            model = data.get("data", [{}])[0].get("id", "unknown")
            return {"online": True, "model": model}
    except Exception:
        pass
    return {"online": False, "model": None}


def clean_db(console: Console) -> bool:
    console.print("[yellow]Cleaning database...[/yellow]")
    try:
        result = subprocess.run(
            [
                "psql", POSTGRES_URL, "-c",
                "TRUNCATE summaries, documents CASCADE;"
            ],
            capture_output=True, text=True, timeout=10,
        )
        if result.returncode == 0:
            console.print("[green]✓[/green] Database cleaned (documents + summaries truncated)")
            return True
        else:
            console.print(f"[red]✗[/red] psql failed: {result.stderr.strip()}")
            return False
    except FileNotFoundError:
        console.print("[red]✗[/red] psql not found — install PostgreSQL client or clean manually")
        return False
    except Exception as e:
        console.print(f"[red]✗[/red] DB cleanup failed: {e}")
        return False


def cancel_ingest(url: str, console: Console) -> None:
    try:
        resp = httpx.post(f"{url}/cancel", timeout=5)
        if resp.status_code == 200:
            data = resp.json()
            if data.get("cancelled", 0) > 0:
                console.print(f"[yellow]Cancelled {data['cancelled']} queued jobs[/yellow]")
    except Exception:
        pass


def clear_redis_counters(url: str, console: Console) -> None:
    """Cancel clears counters, but we also need to ensure a clean slate."""
    cancel_ingest(url, console)


def start_ingest(url: str, path: str, console: Console) -> dict | None:
    try:
        resp = httpx.post(
            f"{url}/ingest",
            json={"path": path},
            timeout=30,
        )
        if resp.status_code == 200:
            return resp.json()
        else:
            console.print(f"[red]✗[/red] Ingest failed: HTTP {resp.status_code} — {resp.text}")
            return None
    except Exception as e:
        console.print(f"[red]✗[/red] Ingest request failed: {e}")
        return None


def get_status(url: str) -> dict:
    """Fetch current queue status. Returns zeroed dict on failure."""
    try:
        resp = httpx.get(f"{url}/status", timeout=5)
        return resp.json()
    except Exception:
        return {"total": 0, "completed": 0, "failed": 0, "in_progress": 0, "errors": []}


def poll_status(url: str, queued: int, baseline: dict, console: Console) -> dict:
    """Poll GET /status until all newly queued files are done. Returns delta results."""
    start = time.monotonic()
    base_completed = baseline.get("completed", 0)
    base_failed = baseline.get("failed", 0)

    with Live(console=console, refresh_per_second=2) as live:
        while True:
            try:
                resp = httpx.get(f"{url}/status", timeout=5)
                status = resp.json()
            except Exception as e:
                live.update(Text(f"  Polling error: {e}", style="red"))
                time.sleep(POLL_INTERVAL)
                continue

            elapsed = time.monotonic() - start
            new_completed = status.get("completed", 0) - base_completed
            new_failed = status.get("failed", 0) - base_failed
            done = new_completed + new_failed

            if queued > 0:
                pct = min(done / queued * 100, 100.0)
                bar_len = 30
                filled = int(bar_len * done / queued)
                bar = "█" * filled + "░" * (bar_len - filled)
                rate = done / elapsed if elapsed > 0 else 0

                line = Text()
                line.append(f"  [{bar}] ", style="cyan")
                line.append(f"{pct:5.1f}%  ", style="bold")
                line.append(f"{new_completed}", style="green")
                line.append(f"+{new_failed}", style="red" if new_failed > 0 else "dim")
                line.append(f"/{queued}  ", style="dim")
                line.append(f"{elapsed:.0f}s  ", style="dim")
                line.append(f"{rate:.1f} files/s", style="yellow")
                live.update(line)

            if done >= queued:
                return {
                    "total": queued,
                    "completed": new_completed,
                    "failed": new_failed,
                    "elapsed_s": time.monotonic() - start,
                    "errors": status.get("errors", []),
                }

            time.sleep(POLL_INTERVAL)


def print_results(results: dict, label: str, console: Console) -> None:
    table = Table(title=f"Ingest Benchmark — {label}", box=box.ROUNDED)
    table.add_column("Metric", style="bold")
    table.add_column("Value", justify="right")

    elapsed = results["elapsed_s"]
    total = results["total"]
    completed = results["completed"]
    failed = results["failed"]
    rate = completed / elapsed if elapsed > 0 else 0

    table.add_row("Total files", str(total))
    table.add_row("Completed", f"[green]{completed}[/green]")
    table.add_row("Failed", f"[red]{failed}[/red]" if failed > 0 else "0")
    table.add_row("Wall-clock time", f"{elapsed:.1f}s")
    table.add_row("Throughput", f"[bold yellow]{rate:.2f} files/s[/bold yellow]")
    table.add_row("Avg time/file", f"{elapsed / completed:.1f}s" if completed > 0 else "—")

    if results.get("files_queued") is not None:
        table.add_row("Files queued", str(results["files_queued"]))
    if results.get("files_skipped") is not None:
        table.add_row("Files skipped (unchanged)", str(results["files_skipped"]))
    if results.get("sidecar_model"):
        table.add_row("Sidecar model", results["sidecar_model"])

    console.print()
    console.print(table)

    if failed > 0 and results.get("errors"):
        console.print("\n[red]First 5 errors:[/red]")
        for err in results["errors"][:5]:
            console.print(f"  [dim]{err.get('file', '?')}[/dim]: {err.get('error', '?')}")


def print_comparison(a: dict, b: dict, console: Console) -> None:
    table = Table(title="Ingest Benchmark Comparison", box=box.ROUNDED, show_header=True)
    table.add_column("Metric", style="bold")
    table.add_column(a.get("label", "Run A"), justify="right")
    table.add_column(b.get("label", "Run B"), justify="right")
    table.add_column("Δ", justify="right")

    def delta(va: float, vb: float, higher_is_better: bool = False) -> str:
        if va == 0:
            return "—"
        pct = (vb - va) / va * 100
        better = pct > 0 if higher_is_better else pct < 0
        color = "green" if better else "red"
        sign = "+" if pct > 0 else ""
        return f"[{color}]{sign}{pct:.1f}%[/{color}]"

    ea, eb = a["elapsed_s"], b["elapsed_s"]
    ca, cb = a["completed"], b["completed"]
    ra = ca / ea if ea > 0 else 0
    rb = cb / eb if eb > 0 else 0

    table.add_row("Total files", str(a["total"]), str(b["total"]), "")
    table.add_row("Completed", str(ca), str(cb), "")
    table.add_row("Failed", str(a["failed"]), str(b["failed"]), "")
    table.add_row("Wall-clock time", f"{ea:.1f}s", f"{eb:.1f}s", delta(ea, eb))
    table.add_row("Throughput (files/s)", f"{ra:.2f}", f"{rb:.2f}", delta(ra, rb, higher_is_better=True))
    table.add_row("Avg time/file", f"{ea/ca:.1f}s" if ca > 0 else "—", f"{eb/cb:.1f}s" if cb > 0 else "—",
                   delta(ea / ca if ca > 0 else 0, eb / cb if cb > 0 else 0))

    if a.get("sidecar_model"):
        table.add_row("Model A", a["sidecar_model"], "", "")
    if b.get("sidecar_model"):
        table.add_row("Model B", "", b["sidecar_model"], "")

    console.print()
    console.print(table)


def count_md_files(path: str) -> int:
    return len(list(Path(path).glob("*.md")))


def select_files(source: str, limit: int, console: Console) -> str:
    """Select exactly `limit` .md files (sorted) and symlink them into a temp dir.

    Returns the absolute path to the temp directory. Caller must clean up.
    """
    all_files = sorted(Path(source).glob("*.md"), key=lambda p: p.name)
    if limit > len(all_files):
        console.print(f"[red]✗ --limit {limit} but only {len(all_files)} .md files in {source}[/red]")
        sys.exit(1)

    selected = all_files[:limit]
    tmp_dir = tempfile.mkdtemp(prefix="hawkeye_bench_")

    for f in selected:
        os.symlink(f.resolve(), Path(tmp_dir) / f.name)

    console.print(f"[green]✓[/green] Selected {limit}/{len(all_files)} files (sorted alphabetically) → [dim]{tmp_dir}[/dim]")
    for i, f in enumerate(selected):
        console.print(f"  [dim]{i+1:3d}.[/dim] {f.name}")

    return tmp_dir


def main() -> None:
    parser = argparse.ArgumentParser(description="Hawkeye ingest pipeline benchmark")
    parser.add_argument("--url", default=HAWKEYE_URL,
                        help=f"Hawkeye server URL (default: {HAWKEYE_URL})")
    parser.add_argument("--path", default="test_data",
                        help="Directory to ingest (default: test_data)")
    parser.add_argument("--clean", action="store_true",
                        help="Truncate documents + summaries tables before ingesting (requires psql)")
    parser.add_argument("--output", help="Write JSON results to file")
    parser.add_argument("--label", help="Label for this run (used in output/comparison)")
    parser.add_argument("--limit", type=int,
                        help="Ingest only the first N files (sorted alphabetically) for reproducible A/B tests")
    parser.add_argument("--compare", nargs=2, metavar=("FILE_A", "FILE_B"),
                        help="Compare two saved JSON result files instead of running a benchmark")
    args = parser.parse_args()

    console = Console()
    console.print("[bold]Hawkeye Ingest Pipeline Benchmark[/bold]\n")

    # Compare mode — just load two JSON files and print comparison
    if args.compare:
        with open(args.compare[0]) as f:
            a = json.load(f)
        with open(args.compare[1]) as f:
            b = json.load(f)
        a.setdefault("label", args.compare[0])
        b.setdefault("label", args.compare[1])
        print_comparison(a, b, console)
        return

    # Pre-flight checks
    console.print("[dim]Pre-flight checks...[/dim]")

    if not check_server(args.url, console):
        console.print(f"[red]✗ Hawkeye server not reachable at {args.url}[/red]")
        console.print("  Start with: cargo run --release")
        sys.exit(1)
    console.print(f"[green]✓[/green] Hawkeye server at {args.url}")

    sidecar_url = "http://localhost:7701"
    sidecar = check_sidecar(sidecar_url)
    if sidecar["online"]:
        console.print(f"[green]✓[/green] Sidecar online — model: [bold]{sidecar['model']}[/bold]")
    else:
        console.print(f"[red]✗ Sidecar not reachable at {sidecar_url}[/red]")
        console.print("  Start with: ./scripts/start_mlx.sh")
        console.print("  Or:         ./scripts/start_vllm_mlx.sh")
        sys.exit(1)

    md_count = count_md_files(args.path)
    console.print(f"[green]✓[/green] {md_count} markdown files in [dim]{args.path}[/dim]")

    # If --limit, create temp dir with symlinks to exactly N sorted files
    ingest_dir = args.path
    tmp_dir = None
    selected_files: list[str] = []
    if args.limit:
        if not args.clean:
            console.print("[yellow]⚠ Using --limit without --clean — consider --clean for fair A/B comparison[/yellow]")
        tmp_dir = select_files(args.path, args.limit, console)
        selected_files = sorted(os.listdir(tmp_dir))
        ingest_dir = tmp_dir
        atexit.register(shutil.rmtree, tmp_dir, True)

    # Clean DB if requested
    if args.clean:
        if not clean_db(console):
            sys.exit(1)

    # Cancel any stale ingest
    clear_redis_counters(args.url, console)

    # Snapshot status before ingest (to compute deltas accurately)
    baseline_status = get_status(args.url)

    # Start ingest
    file_count_label = f"{args.limit} of" if args.limit else "all"
    console.print(f"\n[bold cyan]Starting ingest of {file_count_label} {args.path}...[/bold cyan]")

    ingest_resp = start_ingest(args.url, ingest_dir, console)
    if ingest_resp is None:
        sys.exit(1)

    queued = ingest_resp.get("files_queued", 0)
    skipped = ingest_resp.get("files_skipped", 0)
    console.print(f"  Queued: [bold]{queued}[/bold]  Skipped: [dim]{skipped}[/dim]")

    if queued == 0:
        console.print("\n[yellow]Nothing to ingest — all files already processed.[/yellow]")
        console.print("Use [bold]--clean[/bold] to truncate the DB for a fresh run.")
        return

    # Poll until done (using deltas from baseline)
    console.print()
    final_status = poll_status(args.url, queued, baseline_status, console)

    # Build results (already delta-based from poll_status)
    label = args.label or sidecar.get("model", "unknown")
    results = {
        "label": label,
        "total": final_status["total"],
        "completed": final_status["completed"],
        "failed": final_status["failed"],
        "elapsed_s": final_status["elapsed_s"],
        "files_queued": queued,
        "files_skipped": skipped,
        "sidecar_model": sidecar.get("model"),
        "errors": final_status.get("errors", []),
    }
    if selected_files:
        results["selected_files"] = selected_files

    print_results(results, label, console)

    if args.output:
        with open(args.output, "w") as f:
            json.dump(results, f, indent=2)
        console.print(f"\nResults saved to [dim]{args.output}[/dim]")
        console.print(f"Compare later: uv run scripts/benchmark_ingest.py --compare {args.output} other_results.json")


if __name__ == "__main__":
    main()
