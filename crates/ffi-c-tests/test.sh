#!/bin/bash
set -euo pipefail

GREEN="\033[0;32m"
RED="\033[0;31m"
NC="\033[0m"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR" || exit 1

# Leak detection is split by platform because:
#  - macOS Apple clang ships ASan but not LSan, and `leaks` refuses to inspect
#    ASan-instrumented binaries; so use `leaks` against an unsanitized binary.
#  - Linux clang/gcc ship ASan+LSan together; one ASan-instrumented binary
#    catches both leaks and other memory errors at runtime.
case "$(uname -s)" in
  Darwin)
    CC="${CC:-clang}"
    SAN_FLAGS="-g"
    LEAK_CHECK=(env MallocStackLogging=1 leaks --atExit --)
    ;;
  Linux)
    CC="${CC:-cc}"
    SAN_FLAGS="-fsanitize=address -fno-omit-frame-pointer -g"
    LEAK_CHECK=()
    export ASAN_OPTIONS="detect_leaks=1:halt_on_error=1"
    export LSAN_OPTIONS="exitcode=23"
    ;;
  *)
    echo "Unsupported OS: $(uname -s)"; exit 1 ;;
esac

TARGET_DIR="../../target/debug"

# Build both in one invocation so cargo unifies features (ezffi inherits
# `generics` from ffi-c-tests' dep declaration).
cargo build -p ezffi -p ffi-c-tests

EXIT_CODE=0

rm -rf include
mkdir -p include
cp -r "${TARGET_DIR}/include/." include/
mv include/ffi-c-tests/ffi-c-tests.h include/ffi-c-tests/test.h

for file in c-tests/*.c; do
  if "$CC" $SAN_FLAGS "$file" -Iinclude -L"$TARGET_DIR" -lffi_c_tests -lezffi -o test.bin && "${LEAK_CHECK[@]}" ./test.bin; then
    echo -e "${GREEN}Passed: $file${NC}"
  else
    echo -e "${RED}Failed: $file${NC}"
    EXIT_CODE=1
  fi
done

rm -f test.bin

exit $EXIT_CODE
