# Devices and IO

A [*Process*](./processes.md) can read values from inputs or write values to
outputs using a `Device`.

Inputs and outputs are referred to via a symbolic `InputNumber` or 
`OutputNumber`. This allows a process to say *"Do a digital read from input 5"*
and not care which `Device` actually does the read or how the input is wired
up.
