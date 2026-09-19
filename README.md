# fqtl-rs

Fine-mapping and QTL tooling in Rust.

- Crate / library: `fqtl-rs`
- CLI binary: `fqtl`

See [crates/fqtl-rs/README.md](crates/fqtl-rs/README.md) for subcommands and examples.

This workspace vendors foundation crates (`matrix-util`, `candle-util`, `genomic-data`,
`mcmc-util`, plus `leiden` for `matrix-util`) so the tree builds without
[legume-rs](https://github.com/causalpathlab/legume-rs).

## Build

```bash
cargo build -p fqtl-rs --release
cargo test -p fqtl-rs
```

## License

MIT
