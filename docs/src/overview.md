# Overview

![Overview Diagram](Overview.png)

## Assumptions About the Environment

The [*Runtime*](./runtime.md) requires **a global allocator** (see the [Rust
Embedded Guide][reg] for more) because some level of type erasure is needed
when handling different types of [*Process*](./processes.md) and
[*Devices*](./deviced.md).

[reg]: https://rust-embedded.github.io/book/collections/index.html