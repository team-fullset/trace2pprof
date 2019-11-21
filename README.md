# trace2pprof

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
pprof -tools /path/to/cross/tools/bin -http :8080 test.o profile.pb.gz
```
