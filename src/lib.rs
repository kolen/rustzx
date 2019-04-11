#![allow(dead_code)]

/// Lazy static for macine specs
#[macro_use]
extern crate lazy_static;
/// Command line parser
extern crate clap;
/// backend => sound, video, events
extern crate sdl2;
/// AY chip emulation library pacmancoder/rust-ayumi
extern crate ayumi;

// crate consists of theese modules
mod utils;
mod z80;
mod zx;
mod app;
pub mod emulator;
mod backends;
pub mod settings;
