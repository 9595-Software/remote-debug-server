# remote-debug-server

## Description
Remote debug server is a simple server that allows you to debug your code remotely. It is essentially an http server echoing the request body to stdout.

## Usage
```bash
$ remote-debug-server
```
Point your http client to the IP addresses printed by the `remote-debug-server` command and send a POST request with the debug content as the body.

## Installation
### From source
```bash
cargo build --release
cp target/release/remote-debug-server /usr/local/bin
```
