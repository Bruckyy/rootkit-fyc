# Sample driver

### Prerequisites

```
winget install LLVM.LLVM
```
```
cargo install --locked cargo-make --no-default-features --features tls-native
```
```
rustup toolchain install nightly
rustup default nightly
```

### Compile

Compile the driver
```
cargo make
```

### Test
On the test VM:

Enable loading of test drivers 
```
bcdedit -set TESTSIGNING ON
```
/!\ REBOOT THE MACHINE AFTER THIS COMMAND /!\


Create the service 
```
sc create <service_name> binPath=<path_to_driver.sys> type= kernel
```

Start the service to load the driver
```
sc start <service_name>
```

Stop the service to unload the driver
```
sc stop <service_name>
```

While starting and stopping the driver's service you should see the prints in dbgview (dont forget to enable "capture kernel" and verbose kernel output)