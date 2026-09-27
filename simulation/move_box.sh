#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

LIB_PATH=$(uv run python -c "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))")
export DYLD_LIBRARY_PATH="$LIB_PATH:$DYLD_LIBRARY_PATH"

# 絶対パスで move_box.py を実行
exec "$SCRIPT_DIR/../.venv/bin/mjpython" "$SCRIPT_DIR/move_box.py"
