//! ARM and RISC-V basic utilities and assembly.



#![no_std]

#![deny(warnings)]

#![feature(asm_const)]



#[cfg(target_arch = "arm")]
pub mod arm;


/// Common module to interact with hardware registers implementations.
pub mod reg;
