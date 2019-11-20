# trace2pprof

## Usage

### Step 1: Generate MAME trace

At the point where you would like to start profiling, run the following command in the MAME debugger:

```
trace game.tr,0,noloop
```

### Step 2: Process trace and generate profile

Once you have the trace, you can generate a profile, using the object file that the compiler produced:

```
trace2pprof --object-file test.o --trace-file game.tr
```

### Step 3: Inspect profile

Now you can open the profile inspector in your web browser:

```
pprof -http 0.0.0.0:8080 profile.pb.gz
```
