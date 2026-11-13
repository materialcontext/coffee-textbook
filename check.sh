#!/bin/sh
# Full verification: Rust unit tests, build with checks, JS syntax and logic, DOM tests.
set -e
cd "$(dirname "$0")"
cargo test --quiet
cargo build --release --quiet
./target/release/upstream build content dist
(cd tests/js && [ -d node_modules ] || npm install --silent acorn@8 jsdom@24)
node tests/js/calc.test.js
node tests/js/pages.test.js dist
# Editions: set EPUBCHECK to the path of epubcheck.jar to validate the EPUB too.
if [ -n "$EPUBCHECK" ]; then
  rm -rf target/release-check && mkdir -p target/release-check
  cp -r content target/release-check/
  ./target/release/upstream release target/release-check/content target/release-check/dist --buyer "Check Reader" > /dev/null
  java -jar "$EPUBCHECK" target/release-check/release/*.epub 2>&1 | grep -E "^(ERROR|FATAL|WARNING)|Messages" 
  java -jar "$EPUBCHECK" -q target/release-check/release/*.epub
fi
echo "all checks passed"
