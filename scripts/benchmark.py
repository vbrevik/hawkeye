#!/usr/bin/env python3
"""
Hawkeye inference benchmark — measures throughput with and without speculative decoding.

Usage:
    # Single run (measures current sidecar config):
    uv run scripts/benchmark.py

    # Compare two sidecars explicitly:
    uv run scripts/benchmark.py --baseline http://localhost:7701 --speculative http://localhost:7702

    # More requests for stable numbers:
    uv run scripts/benchmark.py --requests 20

The script sends the hawkeye summarization prompt to the inference endpoint
and records latency, tokens generated, and tokens/second for each request.
"""
# /// script
# requires-python = ">=3.11"
# dependencies = ["httpx", "rich"]
# ///

import argparse
import asyncio
import json
import random
import statistics
import time
from pathlib import Path
from typing import NamedTuple

import httpx
from rich.console import Console
from rich.table import Table
from rich import box

SYSTEM_PROMPT = (
    "You are a document summarizer. Given a markdown document, extract:\n"
    "1. A 2-3 sentence TL;DR summary\n"
    "2. A short title\n"
    "3. Relevant tags (lowercase, max 5)\n"
    "4. Named entities mentioned (people, tools, services, max 10)\n"
    "5. High-level topics (max 3)\n\n"
    'Respond ONLY with valid JSON in this exact format:\n'
    '{"tldr":"...","title":"...","tags":["..."],"entities":["..."],"topics":["..."]}\n'
    "No markdown fences. No explanation. Just the JSON object."
)


class RequestResult(NamedTuple):
    latency_s: float
    prompt_tokens: int
    completion_tokens: int
    total_tokens: int
    ok: bool
    error: str = ""


async def run_request(
    client: httpx.AsyncClient,
    url: str,
    model: str,
    content: str,
    filename: str,
) -> RequestResult:
    payload = {
        "model": model,
        "messages": [
            {"role": "system", "content": SYSTEM_PROMPT},
            {"role": "user", "content": f"Filename: {filename}\n\n{content[:3000]}"},
        ],
        "temperature": 0.1,
        "max_tokens": 256,
    }
    t0 = time.perf_counter()
    try:
        resp = await client.post(f"{url}/v1/chat/completions", json=payload, timeout=120)
        latency = time.perf_counter() - t0
        if resp.status_code != 200:
            return RequestResult(latency, 0, 0, 0, False, f"HTTP {resp.status_code}")
        data = resp.json()
        usage = data.get("usage", {})
        return RequestResult(
            latency_s=latency,
            prompt_tokens=usage.get("prompt_tokens", 0),
            completion_tokens=usage.get("completion_tokens", 0),
            total_tokens=usage.get("total_tokens", 0),
            ok=True,
        )
    except Exception as e:
        return RequestResult(time.perf_counter() - t0, 0, 0, 0, False, str(e))


def get_model_name(url: str) -> str:
    try:
        resp = httpx.get(f"{url}/v1/models", timeout=5)
        data = resp.json()
        return data["data"][0]["id"]
    except Exception:
        return "unknown"


def load_test_docs(test_data_dir: Path, n: int) -> list[tuple[str, str]]:
    """Return up to n (filename, content) pairs from test_data_dir."""
    files = list(test_data_dir.glob("*.md"))
    if not files:
        # Fall back to synthetic documents
        return [(f"doc-{i}.md", f"# Document {i}\n\nSample content for benchmarking.") for i in range(n)]
    random.seed(42)
    chosen = random.sample(files, min(n, len(files)))
    docs = []
    for f in chosen:
        try:
            docs.append((f.name, f.read_text(errors="replace")))
        except OSError:
            pass
    return docs


async def run_benchmark(
    url: str,
    model: str,
    docs: list[tuple[str, str]],
    concurrency: int,
    label: str,
    console: Console,
) -> list[RequestResult]:
    console.print(f"\n[bold cyan]── {label}[/bold cyan]  {url}  model=[dim]{model}[/dim]")
    semaphore = asyncio.Semaphore(concurrency)
    results: list[RequestResult] = []

    async with httpx.AsyncClient() as client:
        async def bounded(filename: str, content: str, idx: int) -> RequestResult:
            async with semaphore:
                result = await run_request(client, url, model, content, filename)
                status = "[green]✓[/green]" if result.ok else "[red]✗[/red]"
                console.print(
                    f"  {status} [{idx+1:3d}/{len(docs)}] "
                    f"{filename[:40]:<40} "
                    f"{result.latency_s:6.2f}s  "
                    f"{result.completion_tokens} tok"
                )
                return result

        tasks = [bounded(fname, content, i) for i, (fname, content) in enumerate(docs)]
        results = await asyncio.gather(*tasks)

    return list(results)


def summarize(results: list[RequestResult]) -> dict:
    ok = [r for r in results if r.ok]
    if not ok:
        return {}
    latencies = [r.latency_s for r in ok]
    comp_toks = [r.completion_tokens for r in ok]
    tps = [r.completion_tokens / r.latency_s for r in ok if r.latency_s > 0]
    return {
        "n": len(results),
        "ok": len(ok),
        "failed": len(results) - len(ok),
        "mean_latency": statistics.mean(latencies),
        "p50_latency": statistics.median(latencies),
        "p95_latency": sorted(latencies)[int(len(latencies) * 0.95)],
        "mean_completion_tokens": statistics.mean(comp_toks),
        "mean_tokens_per_sec": statistics.mean(tps),
        "total_tokens": sum(r.total_tokens for r in ok),
    }


def print_comparison(baseline: dict, spec: dict, console: Console) -> None:
    table = Table(title="Benchmark Results", box=box.ROUNDED, show_header=True)
    table.add_column("Metric", style="bold")
    table.add_column("Baseline (no spec)", justify="right")
    table.add_column("Speculative decoding", justify="right")
    table.add_column("Δ", justify="right")

    def delta(b: float, s: float, higher_is_better: bool = False) -> str:
        if b == 0:
            return "—"
        pct = (s - b) / b * 100
        better = pct > 0 if higher_is_better else pct < 0
        color = "green" if better else "red"
        sign = "+" if pct > 0 else ""
        return f"[{color}]{sign}{pct:.1f}%[/{color}]"

    rows = [
        ("Requests", f"{baseline['n']}", f"{spec['n']}", ""),
        ("Failed", f"{baseline['failed']}", f"{spec['failed']}", ""),
        ("Mean latency", f"{baseline['mean_latency']:.2f}s", f"{spec['mean_latency']:.2f}s",
         delta(baseline['mean_latency'], spec['mean_latency'])),
        ("p50 latency", f"{baseline['p50_latency']:.2f}s", f"{spec['p50_latency']:.2f}s",
         delta(baseline['p50_latency'], spec['p50_latency'])),
        ("p95 latency", f"{baseline['p95_latency']:.2f}s", f"{spec['p95_latency']:.2f}s",
         delta(baseline['p95_latency'], spec['p95_latency'])),
        ("Mean completion tokens", f"{baseline['mean_completion_tokens']:.0f}",
         f"{spec['mean_completion_tokens']:.0f}", ""),
        ("Tokens/sec", f"{baseline['mean_tokens_per_sec']:.1f}",
         f"{spec['mean_tokens_per_sec']:.1f}",
         delta(baseline['mean_tokens_per_sec'], spec['mean_tokens_per_sec'], higher_is_better=True)),
    ]
    for row in rows:
        table.add_row(*row)
    console.print()
    console.print(table)


def print_single(stats: dict, label: str, console: Console) -> None:
    table = Table(title=f"Benchmark Results — {label}", box=box.ROUNDED)
    table.add_column("Metric", style="bold")
    table.add_column("Value", justify="right")
    table.add_row("Requests", str(stats["n"]))
    table.add_row("Failed", str(stats["failed"]))
    table.add_row("Mean latency", f"{stats['mean_latency']:.2f}s")
    table.add_row("p50 latency", f"{stats['p50_latency']:.2f}s")
    table.add_row("p95 latency", f"{stats['p95_latency']:.2f}s")
    table.add_row("Mean completion tokens", f"{stats['mean_completion_tokens']:.0f}")
    table.add_row("Tokens/sec", f"{stats['mean_tokens_per_sec']:.1f}")
    console.print()
    console.print(table)


async def main() -> None:
    parser = argparse.ArgumentParser(description="Hawkeye inference benchmark")
    parser.add_argument("--url", default="http://localhost:7701",
                        help="MLX sidecar URL for single-mode benchmark (default: http://localhost:7701)")
    parser.add_argument("--baseline", help="URL of baseline sidecar (no speculative decoding)")
    parser.add_argument("--speculative", help="URL of speculative sidecar")
    parser.add_argument("--requests", type=int, default=10,
                        help="Number of documents to benchmark (default: 10)")
    parser.add_argument("--concurrency", type=int, default=1,
                        help="Concurrent requests (default: 1 — sequential)")
    parser.add_argument("--test-data", default="test_data",
                        help="Path to test data directory (default: test_data)")
    parser.add_argument("--output", help="Write JSON results to file")
    args = parser.parse_args()

    console = Console()
    console.print("[bold]Hawkeye Inference Benchmark[/bold]")
    console.print(f"Requests: {args.requests}  Concurrency: {args.concurrency}")

    test_data_dir = Path(args.test_data)
    docs = load_test_docs(test_data_dir, args.requests)
    console.print(f"Loaded {len(docs)} documents from [dim]{test_data_dir}[/dim]")

    compare_mode = args.baseline and args.speculative

    if compare_mode:
        baseline_model = get_model_name(args.baseline)
        spec_model = get_model_name(args.speculative)

        baseline_results = await run_benchmark(
            args.baseline, baseline_model, docs, args.concurrency, "Baseline (no speculative decoding)", console
        )
        spec_results = await run_benchmark(
            args.speculative, spec_model, docs, args.concurrency, "Speculative decoding", console
        )

        baseline_stats = summarize(baseline_results)
        spec_stats = summarize(spec_results)
        print_comparison(baseline_stats, spec_stats, console)

        if args.output:
            with open(args.output, "w") as f:
                json.dump({"baseline": baseline_stats, "speculative": spec_stats}, f, indent=2)
            console.print(f"\nResults written to [dim]{args.output}[/dim]")

    else:
        url = args.url
        model = get_model_name(url)
        results = await run_benchmark(url, model, docs, args.concurrency, "Single mode", console)
        stats = summarize(results)
        print_single(stats, model, console)

        if args.output:
            with open(args.output, "w") as f:
                json.dump(stats, f, indent=2)
            console.print(f"\nResults written to [dim]{args.output}[/dim]")


if __name__ == "__main__":
    asyncio.run(main())
