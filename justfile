check:
   RUSTFLAGS="-Awarnings" cargo check

check_all:
   cargo check

run:
   @RUSTFLAGS="-Awarnings" cargo run --quiet -- test.py
