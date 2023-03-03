### JSPlay

A JavaScript REPL based on QuickJS runtime and written in rust.

Support compiling js to binary with Node.js like host environment.

#### Build

```sh
cargo build --release
```

#### Run as REPL

```sh
# add binary to PATH
export PATH="${PWD}/target/release:${PATH}"
# run
jsplay
> 'input your js code'
```

#### Load JS file and exec

```sh
jsplay ./index.js
```

