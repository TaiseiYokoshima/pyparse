check:
   RUSTFLAGS="-Awarnings" cargo check

check_all:
   cargo check

run:
   cargo run -- test.py
