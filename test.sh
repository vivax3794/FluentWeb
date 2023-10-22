cd TestProject
cargo run --quiet --manifest-path ../fluent_web/Cargo.toml
wasm-pack test --headless --chrome --firefox
