# fqtl-rs

**fqtl** — functions for QTL analysis.

- Crate / library: `fqtl-rs`
- CLI binary: `fqtl`

## Build

```bash
cargo build --release
cargo test
```

Optional GPU backends:

```bash
cargo build --release --features metal   # macOS
cargo build --release --features cuda    # Linux + CUDA
```

## Subcommands

### Simulation

- **`sim-geno`** — Wright-Fisher forward simulation of genotypes → PLINK BED
- **`sim-sumstat`** — multi-trait GWAS summary statistics with LD structure
- **`sim-mediation`** — SNP → expression → phenotype with confounders

### Fine-mapping and regression

- **`fit-sumstat-sgvb`** — multi-trait fine-mapping from GWAS z-scores (SuSiE / biSuSiE / spike-slab)
- **`fit-sumstat-mcmc`** — same RSS likelihood via elliptical slice sampling
- **`fit-prs-susie`** — ridge PRS from z-scores, then SuSiE on predicted phenotypes
- **`fit-regression`** — generic SGVB regression (`gaussian|poisson|nb` × `gaussian|susie`)

I/O focuses on PLINK filesets, delimited/parquet matrices, interval BED, and GFF/GTF.
Single-cell count / pseudobulk paths are out of scope.

```bash
fqtl --help
```

## License

MIT
