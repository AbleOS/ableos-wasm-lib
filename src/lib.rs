#![no_std]

mod panic;

extern "C" {
    pub fn kill();
    pub fn empty();
    pub fn exit();
}
