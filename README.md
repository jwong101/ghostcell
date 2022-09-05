# GhostCell

An implementation of the [ghostcell paper](https://plv.mpi-sws.org/rustbelt/ghostcell)
in Rust.

The ghostcell crate contains the actual implementation of the GhostCell. The ghostcell_macro
is a simple attribute macro for using multiple ghost tokens in the same function, since creating
ghost tokens results in some rightward drift.

## TODO

- better documentation
- add methods for mutably borrowing two unaliased GhostCells with the same brand
