# SHA256 hashing written in Rust
Rust representation of the `sha256sum`.

## Testing & Benchmarking
• The "shabytetestvectors" contains the test vectors for the SHA functions provided by the NIST insitute.
We use the first message from the "SHA256LongMsg.rsp" to test our implementation.

Use the following CLI command to dump a test vector (Unix-like systems only):
```echo "[test vector]" | xxd -p -r > [filename].bin
```

Use the following CLI command to view the context of the dumped .bin file in hexadecimal format (Unix-like systems only):
```xxd -p [filepath to the bin] 
```

• Aditionally, you can always pass a file with a data to the `sha256sum [filepath]` command to compare the results.

• I use the Hyperfine tool for benchmarking:
```cargo clean && cargo build --release
hyperfine -N --warmup 4 -r 200 "target/release/bin test_data/test1.bin"
```

• We can compare our program and the "sha256sum" implementation the following way:
```cargo clean && cargo build --release
hyperfine -N --warmup 4 -r 200 "target/release/bin test_data/test1.bin" "sha256sum test_data/test1.bin"
```