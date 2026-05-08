#!/usr/bin/env bash
set -euo pipefail

PORT=8414

exec trunk serve --port "$PORT" "$@"
