#!/bin/sh
set -e

cargo llvm-cov --lcov --output-path lcov.info
rm -rf coverage.last
if [ -d coverage ]; then mv coverage coverage.last; fi
genhtml lcov.info --output-directory coverage
open coverage/index.html
