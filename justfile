check:
   @RUSTFLAGS="-Awarnings" cargo check --quiet

test:
   @RUSTFLAGS="-Awarnings" cargo test --quiet

check_all:
   cargo check

run:
   @RUSTFLAGS="-Awarnings" cargo run --quiet -- test.py
