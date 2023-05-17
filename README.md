# Connect for Rust

[connect.build] is a family of libraries for building APIs with different
languages, full compatibility with gRPC and cURL.

> WARNING: EXPERIMENTAL IMPLEMENTATION

This implementation is very incomplete, to a point that its essentially broken.
I use this as a learning opportunity to learn Rust, but I hope that eventually
this will become THE implementation for connect.build as I cannot find any other
or maybe even adapted by the official connect.build team.

The way that the code operates is insired by [connect-go] project for the
backend.

Contributions welcome but don't expect meaningful reviews 😅.

## Project Objectives

- [ ] Connect code generation
  - [ ] Client generation
  - [ ] Server handler generation
- [ ] Connect client library
  - [ ] Unary calls
  - [ ] Streaming
- [ ] Full Connect compatibility
- [ ] Full gRPC compabibility

Additional goals:

- [ ] Protobuf code generation (might not be part of this repo, but inspired by [connect-es])
  - [ ] [connect-es] Ease of use `struct` types.

## Building

No build instructions for now, but probably `cargo build`. You will probably
also need [buf.build]

[connect-go]: https://github.com/bufbuild/connect-go
[connect-es]: https://github.com/bufbuild/connect-es
[connect.build]: https://connect.build
[buf.build]: https://buf.build
