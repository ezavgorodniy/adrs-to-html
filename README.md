# adr-to-html

The purpose of the tool to generate html out of [architectural decision records](https://adr.github.io/). The idea of the tool is inspired by [https://github.com/thomvaill/log4brains](https://github.com/thomvaill/log4brains).

## Current state of the project

**Right now I am experimenting with libraries and gaining some knowledge on Rust, which is actually the major goal at the moment - get more confidence in Rust** 

## Get started

There are no pipelines around the project - it is only to run locally with ```cargo run``` and ```cargo test```. To run the project simply add into content/src folder list of adr in md format (for example: take some from https://github.com/thomvaill/log4brains/tree/master/docs/adr) and running command ```cargo run``` will generate web site to content/out folder. 

### Prerequisites

Tools is writting on Rust and do not have yet other dependencies. Please follow [Rust install guide](https://www.rust-lang.org/tools/install). I am personally installed it with [homebrew installation](https://formulae.brew.sh/formula/rust) and currently running the application on 

```
cargo 1.75.0
release: 1.75.0
host: aarch64-apple-darwin
libgit2: 1.7.1 (sys:0.18.1 system)
libcurl: 8.4.0 (sys:0.4.68+curl-8.4.0 system ssl:(SecureTransport) LibreSSL/3.3.6)
os: Mac OS 14.2.1 [64-bit]
```
