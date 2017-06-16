#![feature(test)]

extern crate test;
extern crate bus;
extern crate byteorder;
extern crate portaudio;

pub mod plugable;
pub mod types;
pub mod consts;
pub mod envelope;

#[cfg(test)]
mod tests;
