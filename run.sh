#!/bin/sh

# pass the name of the test you want to run as argument
# check test names inside tests/cycles.rs
# for example $ ./run.sh minute_oslo

if ! command -v cargo-watch &> /dev/null; then
    cargo install cargo-watch || exit 1
    sleep 1
fi

RUST_TEST_TIME_INTEGRATION=3601 cargo-watch -q -c -w ./ -x "test -- --nocapture --test $1"
