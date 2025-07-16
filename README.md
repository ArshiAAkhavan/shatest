# shatest
Simple simulation of what I encountered on that day with HUJ-AGHA-SHATERI

## Problem
The code on the master branch contains the faulty implementation.

### Run Simulation
```bash 
cargo test
```
should have an output like this:
```bash
thread 'test::model_check' panicked at src/lib.rs:48:17:
assertion `left != right` failed
  left: 0
 right: 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test::model_check

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

error: test failed, to rerun pass `--lib`

```
### Isolating Failing Scenario
First, enable loom tracker on the project and save states in a file
```bash
LOOM_CHECKPOINT_INTERVAL=1 LOOM_CHECKPOINT_FILE=$PWD/out.json cargo test
```
Now enable loom tracing on the failing scenario(s)
```bash 
LOOM_LOG=debug \
  LOOM_LOCATION=1 \
  LOOM_CHECKPOINT_INTERVAL=1 \
  LOOM_CHECKPOINT_FILE=$PWD/out.json \
  RUSTFLAGS="--cfg loom" cargo test
```
or if you need more debug info, swap `debug` with `trace` in the command above

## Solution
The code on `solution` branch is the fixed code
