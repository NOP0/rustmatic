# Runtime

The Rustmatic `Runtime` is the component in charge of process scheduling and
providing an abstraction over the platform. This platform abstraction
includes giving a [*Process*](processes.md) access to things like things
like:

- [IO](devices.md)
- timing
- communication with the rest of the system via global variables
- spawning other processes

For the Minimum Viable Product, this `Runtime` can be thought of as one big
infinite loop:

```rust
while running {
  read_inputs();
  poll_waiting_processes();
  write_outputs();

  do_background_tasks_until_next_tick();
}
```

Or in diagram form:

![Overview Diagram](Overview.png)
