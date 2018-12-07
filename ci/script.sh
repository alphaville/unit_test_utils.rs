set -euxo pipefail

main() {
    cargo build
    cargo test
    cargo doc --no-deps
}

main