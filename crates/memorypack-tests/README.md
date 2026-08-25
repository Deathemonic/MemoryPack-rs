# `memorypack-tests`

Cross-language compatibility tests for the Rust and .NET MemoryPack implementations.

Run the .NET tests first to create the shared fixtures, then run Rust tests:

```shell
dotnet test --project crates/memorypack-tests/dotnet-tests
cargo test -p memorypack-test
dotnet test --project crates/memorypack-tests/dotnet-tests
```

The second .NET run evaluates the Rust-generated files.
