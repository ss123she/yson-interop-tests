DATA_DIR := "data"
RUST_BIN := "rust_to_go.yson.bin"
GO_BIN := "go_to_rust.yson.bin"

test-all: clean run-rust-write run-go run-rust-read
    @echo "--- Test finished ---"

clean:
    rm -f {{DATA_DIR}}/*.bin
    mkdir -p {{DATA_DIR}}

run-rust-write:
    @echo "--- 1 Rust ---"
    cd rust && cargo run --release

run-go:
    @echo "--- 2 Go ---"
    cd go && go run main.go

run-rust-read:
    @echo "--- 3 Rust ---"
    cd rust && cargo run --release
