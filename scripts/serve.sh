#!/usr/bin/env bash
set -euo pipefail

PORT=8414

exec trunk serve --address 0.0.0.0 --port "$PORT" "$@"
