# An executable that runs on bare metal
This repository pertains to create a freestanding binary (an executable) that has the capability to run on bare metal. To create that executable we need to follow certain steps:

## Steps to create a bare-metal executable:
* Disable standard library
* Define custom panic handler
* Provide language items
* Provide entry point
* Build executable

## Build executable

By default Rust tries to build an executable that directly points to our system’s environment means it will run on your environment only. For example, binary for Linux, .exe for Windows and so on… and the environment is called the host of our system.

We can check the host of our system by using rustc --version --verbose.
As we are using the Linux Operating System, so our host is [x86_64-unknown-linux-gnu]. It means:
```
x86_64 : CPU architecture,
unknown : vendor,
linux : an operating system,
gnu : application binary interface
```

To build our executable for a different environment with no underlying operating system (bare metal environment).

You can use `thumbv7em-none-eabihf` which describes the embedded ARM system. Here none denotes that it has no underlying operating system.

So, before using the target we need to add this in rustup by using this command:
```
rustup target add thumbv7em-none-eabihf
```

For building it we need to add --target with our cargo build, so command should be:
```
cargo build --target thumbv7em-none-eabihf
```
