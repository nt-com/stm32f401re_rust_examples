## STM32F401RE Rust Examples

This repo contains code for my personal minimal working examples of peripheral access based on the crate stm32f4.
It's my basis for personal projects and learning rust/embedded rust. I thought this may be useful for other people, so I made it publically accessible.

### State
Work in Progress. What is done so far:

- GPIO Output
- GPIO Input
- GPIO Input with Interrupt
- 32-bit Timer with Interrupt
- ADC Single Conversion

To follow: I2C, UART, more ADC stuff, SPI, ...

### Dependencies

I'm using these in my Cargo.toml.

```console
[dependencies]
panic-halt = "0.2.0"
cortex-m = "0.7.0"
cortex-m-rt = "0.6.13"

[dependencies.stm32f4]
version = "0.12.1"
features = ["stm32f401", "rt"]
```

### Usage

Check out the Rust Discovery book for general setup. To use the files in this repo, follow the steps below.

```console
cargo new [project name]
cd [project name]
cp PATH_TO_REPO/setup/memory.x .
mkdir .cargo && cp PATH_TO_REPO/setup/config .cargo/
cp PATH_TO_REPO/code/[example]/main.rs src/main.rs
```

cargo build automatically selects the correct ARM platform, 
cargo run automatically selects the correct ARM platform and launches GDB.
Once in GDB do:

```console
target remote :3333
load
break main
continue 	
# you are now at the breakpoint at main
# continue again and you are running main
```

(openOCD must run in the background for cargo run to work...) 

### nt-com
