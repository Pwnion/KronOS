# KronOS
## Description
KronOS is an operating system created from scratch in Rust.

## Target Systems
KronOS currently only targets `x86_64`.

## Notes
### `x86_64` Target Json File
This target file is copied and commented here as JSON does not allow comments.
```json lines
{
  // Target the 'x86_64' architecture, with no particular vendor and running on bare-metal.
  "llvm-target": "x86_64-unknown-none",
  
  // Define the memory layout for data types.
  // We use the same as the 'x86_64-unknown-linux-gnu' target triple.
  "data-layout": "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128",
  
  // Target the 'x86_64' architecture.
  "arch": "x86_64",
  
  // Use little endian, so the least significant byte is stored first in memory.
  "target-endian": "little",
  
  // Pointers should be 64 bits long.
  "target-pointer-width": "64",
  
  // C integers should be 32 bits long.
  "target-c-int-width": "32",
  
  // Run on bare-metal.
  "os": "none",
  
  // Indicate that Rust can compile code into an executable that can be loaded and run on the target hardware.
  // This is opposed to just generating libraries or object files.
  "executables": true,
  
  // Use LLVM's linker flavour 'lld'.
  "linker-flavor": "ld.lld",
  
  // Use the Rust-specific invocation of the LLVM 'lld' linker.
  "linker": "rust-lld",
  
  // Disable stack unwinding, and instead abort on panic.
  "panic-strategy": "abort",
  
  // Disable the red zone.
  "disable-redzone": true,
  
  // We disable the MMX and SSE SIMD instruction sets as using the large SIMD registers in the kernel
  // will lead to performance problems when saving to them and loading from them.
  // Since floating point operations on 'x86_64' require the SIMD registers by default,
  // we also enable the 'soft-float' feature to emulate floating point operations through software
  // functions based on normal integers.
  "features": "-mmx,-sse,+soft-float"
}
```