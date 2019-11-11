#!/usr/bin/env sh
cargo test --package parameterized-macro --test tests expected_ok_all -- --exact && \
cargo test --package parameterized-macro --test tests expected_failures  -- --exact