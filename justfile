check:
   @RUSTFLAGS="-Awarnings" cargo check --quiet

check_all:
   cargo check

run:
   @RUSTFLAGS="-Awarnings" cargo run --quiet -- test.py
