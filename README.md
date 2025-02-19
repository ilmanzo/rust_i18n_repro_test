## Reproducer for the non-deterministic builds when usint rust-i18n

At each build, the non-deterministic ordering of the included translations causes a different binary.

To run:

```
$ cargo -q clean ; cargo build -q --release ; sha1sum target/release/rust_i18n_repro_test 
590e9b2151de0e68a1fec7e0cb6487653b929dc1  target/release/rust_i18n_repro_test

$ cargo -q clean ; cargo build -q --release ; sha1sum target/release/rust_i18n_repro_test 
608e48ce2c1c2ef5ffd37940db7ee20b7076aae6  target/release/rust_i18n_repro_test
```
