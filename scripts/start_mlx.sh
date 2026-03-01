#!/usr/bin/env bash
set -euo pipefail

# Hawkeye MLX Inference Sidecar
# Usage: ./scripts/start_mlx.sh [MODEL [DRAFT_MODEL]]
#
# Starts mlx-lm serve on port 7701.
# Default model: mlx-community/Qwen3-Next-80B-A3B-Instruct-4bit
#
# Speculative decoding (EAGLE-3):
#   Pass a draft model as 2nd arg or set MLX_DRAFT_MODEL env var.
#   For Qwen2.5-7B:  leptonai/EAGLE-Qwen2.5-7B-Instruct
#   For gpt-oss-20b: RedHatAI/gpt-oss-20b-speculator.eagle3
#
# Examples:
#   ./scripts/start_mlx.sh
#   ./scripts/start_mlx.sh mlx-community/Qwen2.5-7B-Instruct-4bit leptonai/EAGLE-Qwen2.5-7B-Instruct
#   MLX_DRAFT_MODEL=leptonai/EAGLE-Qwen2.5-7B-Instruct ./scripts/start_mlx.sh
#
# Set MLX_PORT env var to override port (default 7701).

MODEL="${1:-mlx-community/Qwen3-Next-80B-A3B-Instruct-4bit}"
DRAFT_MODEL="${2:-${MLX_DRAFT_MODEL:-}}"
PORT="${MLX_PORT:-7701}"

echo "=== Hawkeye MLX Sidecar ==="
echo "Model : $MODEL"
if [[ -n "$DRAFT_MODEL" ]]; then
  echo "Draft : $DRAFT_MODEL (speculative decoding enabled)"
else
  echo "Draft : none (standard decoding)"
fi
echo "Port  : $PORT"
echo ""

# Check uv
if ! command -v uv &>/dev/null; then
  echo "ERROR: uv not found. Install with: curl -LsSf https://astral.sh/uv/install.sh | sh"
  exit 1
fi

echo "Starting inference server on port $PORT..."
echo "First run will download the model (~11-22GB). Subsequent runs use cache."
echo ""

DRAFT_ARGS=()
if [[ -n "$DRAFT_MODEL" ]]; then
  DRAFT_ARGS=(--draft-model "$DRAFT_MODEL" --num-draft-tokens 5)
fi

exec uv run --with "mlx-lm>=0.26.3" python3 -m mlx_lm.server \
  --model "$MODEL" \
  --port "$PORT" \
  "${DRAFT_ARGS[@]+"${DRAFT_ARGS[@]}"}"
