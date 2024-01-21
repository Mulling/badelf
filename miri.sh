#!/bin/sh

MIRIFLAGS="-Zmiri-disable-isolation" cargo +nightly miri run --bin cli -- target/release/tui
