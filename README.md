# libCwtch-rs

Rust bindings for [libCwtch](https://git.openprivacy.ca/cwtch.im/libcwtch-go/)

Example echobot in examples/echobot.rs (`cargo run --example echobot` -- assumes tor is on $PATH)

## Building

### Updating libCwtch and bingings.rs with Bindgen 

```
cargo install bindgen
```

libCwtch.so version is specified in build.rs. If updating, also download the corresponding libCwtch.h and delete 
the 'preamble from import "C"' section as it imports headers required for the C lib to compile
but that we don't want to create rust bindings for (like importing stdlib.h). Then:

```
bindgen libCwtch.h -o src/cwtchlib_go/bindings.rs
```

### Todo
