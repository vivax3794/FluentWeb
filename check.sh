# Exit on first fail
set -e 

bash tests.sh

echo --------------------- CLIPPY ------------------------
cargo clippy --quiet -- --deny "warnings"
