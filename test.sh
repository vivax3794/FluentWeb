cd TestProject
cargo run --quiet --manifest-path ../fluent_web/Cargo.toml
wasm-pack test --headless --chrome --firefox | sed \
    -e '/^test.*FAIL/ { /^test result: FAILED/ !s/^.*$/\x1b[31m&\x1b[0m/; }' \
    -e '/^test.*ok/ { /^test result: ok/ !s/^.*$/\x1b[32m&\x1b[0m/; }' \
    -e '/\bpassed\b/ s//\x1b[32m&\x1b[0m/g' \
    -e '/\bok\b/ s//\x1b[32m&\x1b[0m/g' \
    -e '/\bfailed\b/ s//\x1b[31m&\x1b[0m/g' \
    -e '/\FAILED\b/ s//\x1b[31m&\x1b[0m/g' \
    -e '/\bFAIL\b/ s//\x1b[31m&\x1b[0m/g' 
