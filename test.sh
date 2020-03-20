#!/usr/bin/env sh
cargo test --package parameterized-parameterized_macro --test tests expected_ok_all -- --exact && \
cargo test --package parameterized-parameterized_macro --test tests expected_failures  -- --exact