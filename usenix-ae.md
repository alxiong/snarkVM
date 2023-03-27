# USENIX23 Artifact Evaluation README

## Getting Started

First, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
Then, try to run `cargo build --release` to compile the project.

Note that all the following benchmark should be run inside `.integration/` folder.

## Benchmark

To run benchmark for `n`-input-`n`-output DPC transaction, first go to file `./dpc/src/testnet2/instantiated.rs`, Line `80-81`:

```rust
    // You can change the following constants to different transaction dimensions.
    // Currently set to 2x2 transactions.
    // E.g. you can change both configs to 3 for 3x3 transactions.
    const NUM_INPUT_RECORDS: usize = 2;
    const NUM_OUTPUT_RECORDS: usize = 2;
```

### Time & Space complexity

Then run:

```
cargo test dpc_testnet2_integration_test --release -- --nocapture
```

You should see some command line log:

```
running 1 test
ℹ️️ universal_srs size for programs: 20487058 bytes
ℹ️️ indexed vk size for programs: 40905 bytes
ℹ️️ crs size for inner snark: 250108401 bytes
before AHP: constraints: 898189
...
after PC batch check: constraints: 4161158
ℹ️️ crs size for outer snark: 4983616881 bytes
⏱️ DPC::Setup takes 194438 ms
⏱️ All 4 program proof gen takes: 9396 ms
⏱️ Inner proof gen takes: 4274 ms
...
⏱️ Outer proof gen takes: 141945 ms
⏱️ DPC::Execute takes: 155662 ms
⚠️ After Execute, Mem usage: current=xxx KB, peak=xxx KB
ℹ️️ total proof size: 193 + 289 = 482 bytes
⏱️ Inner proof verification takes: 5 ms
⏱️ Outer proof verification takes: 5 ms
⏱️ DPC::Verify takes: 15 ms
test dpc_testnet2_integration_test ... ok
```

`DPC::Execute` is the transaction generation we refer to in [VeriZexe](https://eprint.iacr.org/2022/802.pdf).
Most of snarkVM testnet-2 data reported in Table 2 of [XCZ+22] can be found in these logs.

### R1CS constraint complexity

The other measure is about the number of constraints required for the outer circuit (i.e. the "Constraints" column in Table 2).

Run:

```
cargo test test_testnet2_dpc_execute_constraints -- --nocapture
```

You will find something like (the terminal log is much more verbose than what we need, but you should be able to parse relevant info easily):

```
running 1 test
total number of NoopCircuit constraints: 32051
...
⏱️ DPC::Setup takes 199779 ms
total number of NoopCircuit constraints: 32051
...
Inner circuit num constraints: 418189
...
Outer circuit num constraints: 4235068
=========================================================
test test_testnet2_dpc_execute_constraints ... ok
```
