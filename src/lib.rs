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
pub mod utils;
pub mod z80;
pub mod zx;
pub mod app;
pub mod emulator;
pub mod backends;
pub mod settings;
