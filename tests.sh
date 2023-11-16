#!/bin/bash

# Exit on error
set -e
cargo llvm-cov clean --workspace

# Function to run unit tests
run_unit_tests() {
    echo ------------------------------ UNIT TESTS -------------------------------
    mold -run cargo llvm-cov --no-report nextest
}

# Function to run WASM tests
run_wasm_tests() {
    echo ------------------------------ WASM TESTS -------------------------------
    pushd TestProject

    # It won't let us compile fluent_web if the test project is not valid
    rm -r src
    mkdir -p src
    touch src/main.rs

    mold -run cargo llvm-cov --no-report run -p fluent_web 

    mold -run wasm-pack test --headless --chrome --firefox | sed \
        -e '/^test.*FAIL/ { /^test result: FAILED/ !s/^.*$/\x1b[31m&\x1b[0m/; }' \
        -e '/^test.*ok/ { /^test result: ok/ !s/^.*$/\x1b[32m&\x1b[0m/; }' \
        -e '/\bpassed\b/ s//\x1b[32m&\x1b[0m/g' \
        -e '/\bok\b/ s//\x1b[32m&\x1b[0m/g' \
        -e '/\bfailed\b/ s//\x1b[31m&\x1b[0m/g' \
        -e '/\FAILED\b/ s//\x1b[31m&\x1b[0m/g' \
        -e '/\bFAIL\b/ s//\x1b[31m&\x1b[0m/g' ; test ${PIPESTATUS[0]} -eq 0

    popd
}

# Function to combine coverage
combine_coverage() {
    echo ------------------------------ COMBINE COVERAGE ---------------------------
    cargo llvm-cov report --html
}

# Main script logic
case "$1" in
    unit)
        run_unit_tests
        combine_coverage
        ;;
    wasm)
        run_wasm_tests
        combine_coverage
        ;;
    *)
        # Default case: run all tests
        run_unit_tests
        run_wasm_tests
        combine_coverage
        ;;
esac
