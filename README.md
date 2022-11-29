

## RuNA

**RuNA** is a library for aligning DNA sequences. 
<!-- ![runa logo](misc/logo.png) -->
<img src="misc/logo.png" alt="runa logo" width="200"/>



## Docs 

TODO: link to docs

## Commands 

run all tests
``` 
cargo test 
```

test memory usage of a function (for example dist_2)

```
cargo build --test mem_use
GENOME_DATA=? valgrind --tool=massif ???/mem_use-? dist_2
```
One example might be:
```
GENOME_DATA=./tests/genome_instances_data/ valgrind --tool=massif ./target/debug/dep
s/mem_use-67d566998f108632 dist_2
```

You can use all other standard cargo commands, for example to generate documentation or others.


## Questions

[overleaf](https://www.overleaf.com/project/632486670475fd12235d011c)
