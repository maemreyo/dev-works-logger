#!/bin/bash

eval cargo fmt
eval cargo clippy --all-targets -- -D warnings