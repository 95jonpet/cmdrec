#!/bin/bash
#
# Perform a release.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"

VERSION="${1?'Missing required package version'}"
RELEASE="${2?'Missing required package release number'}"

(
  cd "${SCRIPT_DIR}/.."
  cargo clean
  cargo build --release
)

"${SCRIPT_DIR}/build-linux-packages.sh" \
  "${VERSION}" \
  "${RELEASE}" \
  "${SCRIPT_DIR}/../target/release" \
  "${SCRIPT_DIR}/../target/package"
