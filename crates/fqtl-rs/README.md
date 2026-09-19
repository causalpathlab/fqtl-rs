# fqtl-rs

Fine-mapping and QTL tooling in Rust (extracted from the former `fagioli` crate in
[legume-rs](https://github.com/causalpathlab/legume-rs)).

Binary: **`fqtl`**. I/O focuses on PLINK filesets, delimited/parquet matrices,
interval BED, and GFF/GTF gene annotations. Single-cell count / pseudobulk paths
are intentionally out of scope (handled elsewhere).

## Subcommands

### Simulation

- **`sim-geno`** — Wright-Fisher forward simulation of genotypes → PLINK BED
- **`sim-sumstat`** — multi-trait GWAS summary statistics with LD structure
  - Block-level causal architecture (shared + independent causal SNPs per LD block)
  - Correlated genetic architecture via `--num-genetic-factors`
  - Sparse and polygenic (infinitesimal) heritability components
  - Optional low-rank confounders
  - Marginal OLS summary statistics and within-block LD scores
- **`sim-mediation`** — SNP → expression → phenotype with confounders
  - Cis-eQTL effects on mediator genes, mediated and direct pheno
  - Supports collider bias and winner's-curse scenarios

### Fine-mapping and regression

- **`fit-sumstat-sgvb`** — multi-trait fine-mapping from GWAS z-scores
  - RSS likelihood with rSVD-compressed LD
  - `--model susie | bisusie | spike-slab`
  - `--prior-type single | ash`
  - Optional `--refine` for joint refinement of high-PIP variants across blocks
- **`fit-sumstat-mcmc`** — same RSS likelihood, sampled by elliptical slice sampling
- **`fit-prs-susie`** — ridge PRS from z-scores, then SuSiE on predicted phenotypes
  (`--method cavi` or `sgvb`)
- **`fit-regression`** — generic SGVB regression,
  `--model gaussian|poisson|nb` × `--prior gaussian|susie` (alias: `regression`)

## Build

```bash
cargo build -p fqtl-rs --release
# optional GPU backends:
cargo build -p fqtl-rs --release --features metal   # macOS
cargo build -p fqtl-rs --release --features cuda    # Linux + CUDA
```

## Examples

```bash
fqtl sim-geno \
  --num-individuals 2000 \
  --num-snps 10000 \
  --chromosome 22 \
  --ne 10000 \
  --num-generations 1000 \
  --output ./results/geno

fqtl sim-sumstat \
  --bed-prefix /path/to/genotypes \
  --chromosome 22 \
  --output ./results/sim \
  --num-traits 10 \
  --num-shared-causal 5 \
  --num-independent-causal 3 \
  --num-genetic-factors 2 \
  --h2-sparse 0.4 \
  --h2-polygenic 0.1 \
  --seed 42

fqtl sim-mediation \
  --bed-prefix /path/to/genotypes \
  --chromosome 22 \
  --output ./results/med \
  --num-genes 200 \
  --num-mediator-genes 20 \
  --seed 42

fqtl fit-sumstat-sgvb \
  --sumstat-file ./results/sim.sumstats.bed.gz \
  --bed-prefix /path/to/genotypes \
  --chromosome 22 \
  --output ./results/map \
  --model susie \
  --num-components 10 \
  --seed 42

fqtl fit-sumstat-mcmc \
  --sumstat-file ./results/sim.sumstats.bed.gz \
  --bed-prefix /path/to/genotypes \
  --chromosome 22 \
  --output ./results/mcmc \
  --num-components 10

fqtl fit-prs-susie \
  --sumstat-file ./results/sim.sumstats.bed.gz \
  --bed-prefix /path/to/genotypes \
  --chromosome 22 \
  --output ./results/prs \
  --method cavi \
  --num-components 10

fqtl fit-regression \
  -x design.parquet \
  -y outcome.parquet \
  --model gaussian \
  --prior susie \
  --iters 1000 \
  --output ./results/reg

fqtl --help
```

## License

MIT
