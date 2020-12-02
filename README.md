## ftml

<p>
  <a href="https://github.com/Nu-SCPTheme/ftml/actions?query=workflow%3A%22Rust+CI%22">
    <img src="https://github.com/Nu-SCPTheme/ftml/workflows/Rust%20CI/badge.svg"
         alt="Rust CI badge">
  </a>
</p>

Special branch to benchmark stuff. Shouldn't be merged into master.

Well, at least as long as bencher stays nightly-only. If it gets stabilized we can fix up this branch and merge it.

### Benchmarking

To run benchmarks:

```
cargo +nightly bench
```

It's that easy.
