# Wasmer module with a stack trace

Building the guest (can be skipped, the result is checked in):
```
cargo build --manifest-path guest/Cargo.toml --release --target wasm32-unknown-unknown
cp guest/target/wasm32-unknown-unknown/release/stackpanic.wasm host/
```

Running the host:
```
cargo run --manifest-path host/Cargo.toml
```

The expected output:
```
RuntimeError: unreachable
	<module>::rust_begin_unwind
	<module>::core::panicking::panic_fmt
	<module>::core::panicking::panic
	<module>::stackpanic::c
	<module>::stackpanic::b
	<module>::a
```
