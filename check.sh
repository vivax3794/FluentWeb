# Exit on first fail
set -e 

cargo clippy --quiet -- --deny "warnings"
bash test.sh
