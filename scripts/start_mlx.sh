#!/usr/bin/env bash
set -euo pipefail

# Hawkeye MLX Inference Sidecar
# Usage: ./scripts/start_mlx.sh [MODEL [DRAFT_MODEL]]
#
# Starts mlx-lm serve on port 7701.
# Default model: mlx-community/Qwen2.5-7B-Instruct-4bit
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

MODEL="${1:-mlx-community/Qwen2.5-7B-Instruct-4bit}"
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

# Check Python 3
if ! command -v python3 &>/dev/null; then
  echo "ERROR: python3 not found. Install Python 3.10+ from https://python.org"
  exit 1
fi

PYTHON_VERSION=$(python3 -c "import sys; print(f'{sys.version_info.major}.{sys.version_info.minor}')")
echo "Python: $PYTHON_VERSION"

# Check / install mlx-lm (>= 0.26.3 required for gpt_oss architecture)
if ! python3 -c "import mlx_lm" 2>/dev/null; then
  echo "mlx-lm not found — installing..."
  pip3 install "mlx-lm>=0.26.3"
fi

MLX_VERSION=$(python3 -c "import mlx_lm; print(mlx_lm.__version__)" 2>/dev/null || echo "unknown")
echo "mlx-lm: $MLX_VERSION"
echo ""

echo "Starting inference server on port $PORT..."
echo "First run will download the model (~11-22GB). Subsequent runs use cache."
echo ""

DRAFT_ARGS=()
if [[ -n "$DRAFT_MODEL" ]]; then
  DRAFT_ARGS=(--draft-model "$DRAFT_MODEL" --num-draft-tokens 5)
fi

exec python3 -m mlx_lm.server \
  --model "$MODEL" \
  --port "$PORT" \
  "${DRAFT_ARGS[@]}"
