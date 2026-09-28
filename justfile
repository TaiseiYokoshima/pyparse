check:
   @RUSTFLAGS="-Awarnings" cargo check --quiet

# test *args:
#   @RUSTFLAGS="-Awarnings" cargo test --quiet {{args}}

test:
   @RUSTFLAGS="-Awarnings" cargo test --quiet


check_all:
   cargo check

run:
   @RUSTFLAGS="-Awarnings" cargo run --quiet -- test.py
