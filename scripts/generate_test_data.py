#!/usr/bin/env python3
"""
Generate 1000 real markdown files from Wikipedia random articles.
Uses Wikipedia REST API + asyncio for fast parallel fetching.

Usage:
    python3 scripts/generate_test_data.py [OUTPUT_DIR]
    python3 scripts/generate_test_data.py test_data/
"""

import asyncio
import json
import os
import re
import sys
import urllib.request
import urllib.error
from pathlib import Path

OUTPUT_DIR = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("test_data")
TARGET_COUNT = 1000
CONCURRENCY = 20  # parallel requests
USER_AGENT = "hawkeye-test-data/1.0 (https://github.com/local/hawkeye; test data generation)"


def slugify(title: str) -> str:
    """Convert article title to safe filename."""
    s = title.lower().strip()
    s = re.sub(r"[^\w\s-]", "", s)
    s = re.sub(r"[\s_-]+", "-", s)
    s = re.sub(r"^-+|-+$", "", s)
    return s[:80]  # cap length


def fetch_random_titles(batch_size: int = 500) -> list[str]:
    """Fetch random Wikipedia article titles in one batch."""
    url = (
        f"https://en.wikipedia.org/w/api.php"
        f"?action=query&list=random&rnnamespace=0"
        f"&rnlimit={batch_size}&format=json&formatversion=2"
    )
    req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(req, timeout=30) as resp:
        data = json.loads(resp.read())
    return [page["title"] for page in data["query"]["random"]]


def fetch_article(title: str) -> dict | None:
    """Fetch article summary from Wikipedia REST API."""
    encoded = urllib.parse.quote(title.replace(" ", "_"), safe="")
    url = f"https://en.wikipedia.org/api/rest_v1/page/summary/{encoded}"
    req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    try:
        with urllib.request.urlopen(req, timeout=20) as resp:
            return json.loads(resp.read())
    except (urllib.error.HTTPError, urllib.error.URLError, json.JSONDecodeError):
        return None


def article_to_markdown(data: dict) -> str:
    """Convert Wikipedia summary API response to markdown."""
    title = data.get("title", "Untitled")
    description = data.get("description", "")
    extract = data.get("extract", "")
    url = data.get("content_urls", {}).get("desktop", {}).get("page", "")

    lines = [f"# {title}"]
    if description:
        lines.append(f"\n_{description}_\n")
    if extract:
        lines.append(extract)
    if url:
        lines.append(f"\n---\nSource: {url}")
    return "\n".join(lines)


import urllib.parse


async def fetch_article_async(title: str, semaphore: asyncio.Semaphore) -> dict | None:
    """Async wrapper — runs blocking fetch in executor."""
    async with semaphore:
        loop = asyncio.get_event_loop()
        return await loop.run_in_executor(None, fetch_article, title)


async def main():
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    # Check existing files to allow resuming
    existing = set(p.stem for p in OUTPUT_DIR.glob("*.md"))
    print(f"Output dir: {OUTPUT_DIR}/")
    print(f"Already have: {len(existing)} files")

    needed = TARGET_COUNT - len(existing)
    if needed <= 0:
        print(f"Already have {len(existing)} files. Done.")
        return

    print(f"Need to fetch: {needed} more articles")

    all_titles: list[str] = []
    # Fetch in batches of 500 (API max)
    while len(all_titles) < needed + 500:  # overfetch to account for failures/stubs
        batch_size = min(500, needed + 500 - len(all_titles))
        print(f"Fetching {batch_size} random titles...")
        titles = fetch_random_titles(batch_size)
        all_titles.extend(titles)

    semaphore = asyncio.Semaphore(CONCURRENCY)
    saved = 0
    attempted = 0

    tasks = [fetch_article_async(title, semaphore) for title in all_titles]

    print(f"Fetching {len(tasks)} articles ({CONCURRENCY} concurrent)...")

    for coro in asyncio.as_completed(tasks):
        if saved >= needed:
            break

        data = await coro
        attempted += 1

        if not data:
            continue

        title = data.get("title", "")
        extract = data.get("extract", "")

        # Skip completely empty articles only
        if len(extract) < 50:
            continue

        slug = slugify(title)
        if not slug or slug in existing:
            continue

        filename = OUTPUT_DIR / f"{slug}.md"
        if filename.exists():
            continue

        content = article_to_markdown(data)
        filename.write_text(content, encoding="utf-8")
        existing.add(slug)
        saved += 1

        if saved % 50 == 0 or saved == needed:
            total_now = len(list(OUTPUT_DIR.glob("*.md")))
            print(f"  [{saved}/{needed}] saved — total files: {total_now}")

    total_final = len(list(OUTPUT_DIR.glob("*.md")))
    print(f"\nDone! {total_final} markdown files in {OUTPUT_DIR}/")
    print(f"(Attempted {attempted} articles, saved {saved} new)")


if __name__ == "__main__":
    asyncio.run(main())
