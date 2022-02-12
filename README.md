# FastLZ-rs

A port of [FastLZ](https://github.com/ariya/FastLZ) to Rust.

## Testing

Running `cargo test` execute the tests originally in `tests/test_roundtrip.c`.
To obtain a usefull stdout to compare with the C lib you can, for the moment, use:

```
cargo test -- --test-threads=1 --show-output
```

## TODO

 - [x] add a -sys crate to cross-test the porting against the C implementation
 - [ ] automate Rust vs C roundtrip test results comparison
 - [ ] port the actual code
