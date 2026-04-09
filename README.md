# Your_OS

A hobby operating system kernel written in Rust, built from the ground up as an independent learning project. This is not a production kernel, it is an attempt to understand what actually happens beneath every operating system, at the level of hardware and memory.

## Why Rust

Most kernels are written in C, which gives you full control over memory but no safety guarantees. A single bad pointer can corrupt memory silently, and these kinds of bugs have caused real vulnerabilities in production kernels for decades.

Rust offers something different. Its ownership and borrow checker eliminate entire classes of memory bugs at compile time, with no garbage collector and no runtime overhead. This makes it genuinely interesting for kernel development: you get the low-level control that kernel code demands, while the compiler actively prevents the mistakes that make systems software dangerous.

This is also why the Linux kernel began accepting Rust drivers in 2022. It is not a trend, it is a real shift in how systems software can be written.

## How it is being built

This project follows the Blog OS series by Philipp Oppermann, which walks through building a kernel in Rust from scratch — no standard library, no OS underneath, just the hardware. Every concept is implemented and understood before moving forward.

The environment runs on bare metal, meaning none of Rust's standard library is available. Everything that a normal program takes for granted- printing to screen, memory allocation, even panics has to be built by hand.

## Where things are right now

The kernel can boot and write to the screen through a VGA text buffer implementation. This involves writing directly to memory-mapped hardware at address 0xb8000, handling volatile writes so the compiler does not optimize them away, and wrapping the unsafe core in a safe API that the rest of the kernel can use cleanly.

It is a small thing on the surface but it covers a lot of ground. raw pointer manipulation, unsafe Rust, memory-mapped I/O, and Rust's type system being used to make something inherently unsafe safe to work with.

## Where it is going

The planned next steps follow the natural progression of kernel development:

- CPU exceptions and interrupt handling
- Hardware interrupts and the Programmable Interrupt Controller
- Virtual memory and paging
- Heap allocation
- Async and cooperative multitasking

Each of these builds on the last, and the goal is to understand each one properly rather than rush through them.

## Building and running

This project uses QEMU to run the kernel in a virtual machine. With Rust and QEMU installed:

```
cargo run
```

The bootimage tool handles building the kernel and packaging it into a bootable disk image automatically.
