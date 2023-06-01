# RustOnAzure

## Run locally

1. Build: `cargo build --release`
1. Copy executable to root: `cp target/release/app .`
1. Run with Azure CLI: `func start`


## Deploy
1. Build: `TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target x86_64-unknown-linux-musl`
1. Copy executable to root: `cp target/x86_64-unknown-linux-musl/release/app .`
1. Publish: `func azure functionapp publish RustOnAzure`
