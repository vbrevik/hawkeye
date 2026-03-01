#!/usr/bin/env bash
set -euo pipefail

# Hawkeye vllm-mlx Inference Sidecar
# Usage: ./scripts/start_vllm_mlx.sh [MODEL]
#
# Starts vllm-mlx serve on port 7701 with continuous batching.
# Default model: mlx-community/Qwen2.5-7B-Instruct-4bit
#
# vllm-mlx is an MLX-native inference server with continuous batching,
# paged KV cache, and higher throughput than mlx-lm on Apple Silicon.
# GitHub: https://github.com/waybarrios/vllm-mlx
#
# Examples:
#   ./scripts/start_vllm_mlx.sh
#   ./scripts/start_vllm_mlx.sh mlx-community/Qwen3-30B-A3B-Instruct-4bit
#   VLLM_BATCH=false ./scripts/start_vllm_mlx.sh
#
# Environment variables:
#   MLX_PORT       — override port (default 7701)
#   VLLM_BATCH     — enable continuous batching (default true)
#   VLLM_COMMIT    — pin to a specific git commit (default: latest)

MODEL="${1:-mlx-community/Qwen2.5-7B-Instruct-4bit}"
PORT="${MLX_PORT:-7701}"
BATCH="${VLLM_BATCH:-true}"
COMMIT="${VLLM_COMMIT:-}"

REPO="https://github.com/waybarrios/vllm-mlx.git"
if [[ -n "$COMMIT" ]]; then
  REPO="${REPO}@${COMMIT}"
fi

echo "=== Hawkeye vllm-mlx Sidecar ==="
echo "Model   : $MODEL"
echo "Port    : $PORT"
echo "Batching: $BATCH"
if [[ -n "$COMMIT" ]]; then
  echo "Commit  : $COMMIT"
fi
echo ""

# Check uv
if ! command -v uv &>/dev/null; then
  echo "ERROR: uv not found. Install with: curl -LsSf https://astral.sh/uv/install.sh | sh"
  exit 1
fi

echo "Starting vllm-mlx server on port $PORT..."
echo "First run will install vllm-mlx and download model weights."
echo ""

EXTRA_ARGS=()
if [[ "$BATCH" == "true" ]]; then
  EXTRA_ARGS+=(--continuous-batching)
fi

exec uv tool run --from "git+${REPO}" \
  vllm-mlx serve "$MODEL" \
  --port "$PORT" \
  "${EXTRA_ARGS[@]+"${EXTRA_ARGS[@]}"}"
