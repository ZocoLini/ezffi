#!/bin/bash
set -euo pipefail

GREEN="\033[0;32m"
RED="\033[0;31m"
NC="\033[0m"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR" || exit 1

cargo build -p test-lib

EXIT_CODE=0

for file in c-tests/*.c; do
  if gcc "$file" -Iinclude -L../../target/debug -ltest_lib -o test.bin -g && ./test.bin; then
    echo -e "${GREEN}Passed: $file${NC}"
  else
    echo -e "${RED}Failed: $file${NC}"
    EXIT_CODE=1
  fi
done

rm -f test.bin

exit $EXIT_CODE