# Overview

## Assumptions About the Environment

The [*Runtime*](./runtime.md) requires **a global allocator** (see the [Rust
Embedded Guide][reg] for more) because some level of type erasure is needed
when handling different types of [*Process*](./processes.md) and
[*Devices*](./devices.md).

In terms of system requirements, it is assumed that the PLC runtime will run on
top of an Operating System and that the platform will have a "decent" amount of
memory and processing power available. A *Raspberry Pi 1 Model B* should be 
suffucient.

[reg]: https://rust-embedded.github.io/book/collections/index.html