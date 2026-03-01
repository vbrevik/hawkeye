#!/usr/bin/env bash
set -euo pipefail

# Hawkeye Embedding Sidecar — bge-m3 via sentence-transformers + FastAPI
# Usage: ./scripts/start_embed.sh [MODEL]
#
# Starts OpenAI-compatible embedding server on port 7703.
# Default model: BAAI/bge-m3 (1024 dimensions)
#
# Examples:
#   ./scripts/start_embed.sh
#   ./scripts/start_embed.sh BAAI/bge-m3
#   EMBED_PORT=8080 ./scripts/start_embed.sh
#
# Set EMBED_PORT env var to override port (default 7703).
# Set EMBED_MODEL env var to override model.

export EMBED_MODEL="${1:-${EMBED_MODEL:-BAAI/bge-m3}}"
export EMBED_PORT="${EMBED_PORT:-7703}"

echo "=== Hawkeye Embedding Sidecar ==="
echo "Model : $EMBED_MODEL"
echo "Port  : $EMBED_PORT"
echo ""

# Check uv
if ! command -v uv &>/dev/null; then
  echo "ERROR: uv not found. Install with: curl -LsSf https://astral.sh/uv/install.sh | sh"
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Starting embedding server on port $EMBED_PORT..."
echo "First run will download the model. Subsequent runs use cache."
echo ""

exec uv run --python 3.12 \
  --with "sentence-transformers" \
  --with "fastapi" \
  --with "uvicorn" \
  "$SCRIPT_DIR/embed_server.py"
