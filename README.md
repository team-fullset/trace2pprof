# trace2pprof

## What it does

The `trace2pprof` program parses traces generated by [MAME](https://www.mamedev.org/), and converts them into profiles that can be processed using the [`pprof`](https://github.com/google/pprof) tool.

This allows you to use the full suite of `pprof` features (cli, interactive explorer, source annotation) to understand the performance of your application.

## Prerequisites

In order to build and use this tool, you need to have:

- a recent Rust toolchain (available via https://rustup.rs/)
- an installation of pprof (available via `go get` from https://github.com/google/pprof)
- a version of binutils for your target platform (e.g. m68k)

## Build

Running the following command will build the `trace2pprof` tool and install it on your machine:

```
cargo install --path 
```

## Usage

### Step 0: Compile program

Compile your program while passing the `-g` flag to `CC`, in order to emit debug information. Then create the ROM as usual.

### Step 1: Generate MAME trace

At the point where you would like to start profiling, run the following command in the MAME debugger:

```
trace game.tr,0,noloop
```

### Step 2: Process trace and generate profile

Once you have the trace, you can generate a profile:

```
trace2pprof game.tr
```

### Step 3: Inspect profile

Now you can open the profile inspector in your web browser:

```
pprof -tools /path/to/cross/binutils/bin -http :8080 test.o profile.pb.gz
```
