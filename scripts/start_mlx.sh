#!/usr/bin/env bash
set -euo pipefail

# Eagle3 MLX Inference Sidecar
# Usage: ./scripts/start_mlx.sh [MODEL]
#
# Starts mlx-lm serve with GPT-OSS 20B (4-bit) on port 7701.
# Default: mlx-community/gpt-oss-20b-mlx-q8 (better quality, needs ~22GB RAM)
# Fast:    InferenceIllusionist/gpt-oss-20b-MLX-4bit (~11GB RAM)
#
# Set MLX_PORT env var to override port (default 7701).

MODEL="${1:-mlx-community/gpt-oss-20b-mlx-q8}"
PORT="${MLX_PORT:-7701}"

echo "=== Eagle3 MLX Sidecar ==="
echo "Model : $MODEL"
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

exec python3 -m mlx_lm.server \
  --model "$MODEL" \
  --port "$PORT"
