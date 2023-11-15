# Exit on eror
set -e

echo ------------------------------ WASM TESTS -------------------------------
pushd TestProject

# It wont let us compile fluent_web if the test project is not valid
mkdir -p src
touch src/main.rs

RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="../cov_profs/wasm_run.profraw" cargo run -p fluent_web --target x86_64-unknown-linux-gnu

wasm-pack test --headless --chrome --firefox | sed \
    -e '/^test.*FAIL/ { /^test result: FAILED/ !s/^.*$/\x1b[31m&\x1b[0m/; }' \
    -e '/^test.*ok/ { /^test result: ok/ !s/^.*$/\x1b[32m&\x1b[0m/; }' \
    -e '/\bpassed\b/ s//\x1b[32m&\x1b[0m/g' \
    -e '/\bok\b/ s//\x1b[32m&\x1b[0m/g' \
    -e '/\bfailed\b/ s//\x1b[31m&\x1b[0m/g' \
    -e '/\FAILED\b/ s//\x1b[31m&\x1b[0m/g' \
    -e '/\bFAIL\b/ s//\x1b[31m&\x1b[0m/g' ; test ${PIPESTATUS[0]} -eq 0

popd


echo ------------------------------ HANDLE COVERAGE ---------------------------


rm -rv ./cov_profs
