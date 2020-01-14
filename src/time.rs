#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Rate(f32);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Duration(u32);

impl Rate {
    pub fn from_hz(hz: f32) -> Self {
        Self(hz)
    }

    pub fn as_hz(self) -> f32 {
        self.0
    }
}

impl Duration {
    pub fn from_nanos(nanos: u32) -> Self {
        Self(nanos)
    }

    pub fn from_millis(millis: u32) -> Self {
        Self(millis * 1_000)
    }

    pub fn from_seconds(seconds: u32) -> Self {
        Self(seconds * 1_000_000)
    }

    pub fn as_nanos(self) -> u32 {
        self.0
    }
}

pub trait F32Ext {
    fn hz(self) -> Rate;
}

impl F32Ext for f32 {
    fn hz(self) -> Rate {
        Rate(self)
    }
}

pub trait U32Ext {
    fn hz(self) -> Rate;
}

impl U32Ext for u32 {
    fn hz(self) -> Rate {
        Rate(self as f32)
    }
}
