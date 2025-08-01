#!/usr/bin/env bash
cargo build

mkdir -pv cgi-bin
cp -v target/debug/seshanxyz_rust9x cgi-bin/seshanxyz_rust9x


echo "Starting HTTP server on http://localhost:8000/cgi-bin/seshanxyz_rust9x/"
python3 -m http.server --bind localhost --cgi 8000
