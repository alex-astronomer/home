#!/bin/bash

VSCODE_WS="$1"
SSH_REMOTE="$2"
GDBPORT="$3"

APP="rust"
TARGET_ARCH="armv7-unknown-linux-gnueabihf"
BUILD_BIN_FILE="${VSCODE_WS}/target/${TARGET_ARCH}/debug/${APP}"
TARGET_USER="pi"
TARGET_BIN_FILE="/home/pi/${APP}"
TARGET_CWD="/home/pi"

ssh "${TARGET_USER}@${SSH_REMOTE}" "killall gdbserver ${APP}"

if ! rsync -avz "${BUILD_BIN_FILE}" "${TARGET_USER}@${SSH_REMOTE}:${TARGET_BIN_FILE}"; then
    # If rsync doesn't work, it may not be available on target. Fallback to trying SSH copy.
    if ! scp "${BUILD_BIN_FILE}" "${TARGET_USER}@${SSH_REMOTE}:${TARGET_BIN_FILE}"; then
        exit 2
    fi
fi

ssh -f "${TARGET_USER}@${SSH_REMOTE}" "sh -c 'cd ${TARGET_CWD}; nohup gdbserver *:${GDBPORT} ${TARGET_BIN_FILE} > /dev/null 2>&1 &'"
