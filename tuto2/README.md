# Phala tutorial 2

How to query a subquery/subsquid index
https://polkaverse.com/11178/2-how-to-query-an-subquery-subsquid-index-38215

## Build the contract

Setup the environment for Ink! contract compilation, then run

```bash
cd ./query_indexer
cargo contract build
```

## Test

To test your contract locally and see its output, run with

```bash
cd ./query_indexer
cargo test -- --nocapture
```
