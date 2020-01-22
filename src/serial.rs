use crate::io;

pub trait SerialRead: io::Read {}

pub trait SerialWrite: io::Write {}
