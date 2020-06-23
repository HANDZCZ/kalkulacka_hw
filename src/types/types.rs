use std::mem;
use std::convert::TryInto;
use std::fmt;
use fmt::{Display, Formatter};

#[cfg_attr(feature = "debug", derive(Debug))]
pub struct u64x1(u64);

impl From<u32x2> for u64x1 {
    fn from(data: u32x2) -> Self {
        u64x1(unsafe { mem::transmute::<[u32; 2], u64>(data.0) })
    }
}

impl From<u16x4> for u64x1 {
    fn from(data: u16x4) -> Self {
        u64x1(unsafe { mem::transmute::<[u16; 4], u64>(data.0) })
    }
}

impl From<u8x8> for u64x1 {
    fn from(data: u8x8) -> Self {
        u64x1(unsafe { mem::transmute::<[u8; 8], u64>(data.0) })
    }
}

impl u64x1 {
    pub fn new(data: u64) -> Self {
        Self(data)
    }

    pub fn overflowing_add(&self, x: &u64x1) -> u64x1 {
        u64x1(self.0.overflowing_add(x.0).0)
    }

    pub fn saturating_add(&self, x: &u64x1) -> u64x1 {
        u64x1(self.0.saturating_add(x.0))
    }

    pub fn overflowing_increment(&self) -> Self {
        Self(self.0.overflowing_add(1).0)
    }

    pub fn saturating_increment(&self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

impl Display for u64x1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:016X}", self.0)
    }
}

#[cfg_attr(feature = "debug", derive(Debug))]
pub struct u32x2([u32; 2]);

impl From<u64x1> for u32x2 {
    fn from(data: u64x1) -> Self {
        u32x2(unsafe { mem::transmute::<u64, [u32; 2]>(data.0) })
    }
}

impl Display for u32x2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:016X}", self.to_u64x1().0)
    }
}

impl u32x2 {
    pub fn new(data: [u32; 2]) -> Self {
        Self(data)
    }

    pub fn to_u64x1(&self) -> u64x1 {
        u64x1(unsafe { mem::transmute::<[u32; 2], u64>(self.0) })
    }

    pub fn overflowing_add(&self, x: &u32x2) -> u32x2 {
        u32x2(
            self.0.iter()
                .zip(x.0.iter())
                .map(|(x, y)| x.overflowing_add(*y).0)
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .expect("Overflowing addition - Conversion from [u32; X] to [u32; 2] failed.")
        )
    }

    pub fn saturating_add(&self, x: &u32x2) -> u32x2 {
        u32x2(
            self.0.iter()
                .zip(x.0.iter())
                .map(|(x, y)| x.saturating_add(*y))
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .expect("Saturating addition - Conversion from [u32; X] to [u32; 2] failed.")
        )
    }

    pub fn overflowing_increment(&self) -> Self {
        Self(
            self.0.iter()
                .map(|x| x.overflowing_add(1).0)
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .expect("Increment - Conversion from [u32; X] to [u32; 2] failed.")
        )
    }

    pub fn saturating_increment(&self) -> Self {
        Self(
            self.0.iter()
                .map(|x| x.saturating_add(1))
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .expect("Increment - Conversion from [u32; X] to [u32; 2] failed.")
        )
    }

    pub fn multiply(&self, x: &u32x2) -> u64x1 {
        u64x1(self.0[1] as u64 * x.0[1] as u64)
    }
}

#[cfg_attr(feature = "debug", derive(Debug))]
pub struct u16x4([u16; 4]);

impl From<u64x1> for u16x4 {
    fn from(data: u64x1) -> Self {
        u16x4(unsafe { mem::transmute::<u64, [u16; 4]>(data.0) })
    }
}

impl Display for u16x4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:016X}", self.to_u64x1().0)
    }
}

impl u16x4 {
    pub fn new(data: [u16; 4]) -> Self {
        Self(data)
    }

    pub fn to_u64x1(&self) -> u64x1 {
        u64x1(unsafe { mem::transmute::<[u16; 4], u64>(self.0) })
    }

    pub fn overflowing_add(&self, x: &u16x4) -> u16x4 {
        u16x4(
            self.0.iter()
                .zip(x.0.iter())
                .map(|(x, y)| x.overflowing_add(*y).0)
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .expect("Overflowing addition - Conversion from [u16; X] to [u16; 4] failed.")
        )
    }

    pub fn saturating_add(&self, x: &u16x4) -> u16x4 {
        u16x4(
            self.0.iter()
                .zip(x.0.iter())
                .map(|(x, y)| x.saturating_add(*y))
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .expect("Saturating addition - Conversion from [u16; X] to [u16; 4] failed.")
        )
    }

    pub fn overflowing_increment(&self) -> Self {
        Self(
            self.0.iter()
                .map(|x| x.overflowing_add(1).0)
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .expect("Increment - Conversion from [u16; X] to [u16; 4] failed.")
        )
    }

    pub fn saturating_increment(&self) -> Self {
        Self(
            self.0.iter()
                .map(|x| x.saturating_add(1))
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .expect("Increment - Conversion from [u16; X] to [u16; 4] failed.")
        )
    }

    pub fn multiply(&self, x: &u16x4) -> u32x2 {
        u32x2(
            self.0.iter()
                .zip(x.0.iter())
                .step_by(2)
                .map(|(x, y)| *x as u32 * *y as u32)
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .expect("Multiply - Conversion from [u32; X] to [u32; 2] failed.")
        )
    }
}

#[cfg_attr(feature = "debug", derive(Debug))]
pub struct u8x8([u8; 8]);

impl From<u64x1> for u8x8 {
    fn from(data: u64x1) -> Self {
        u8x8(unsafe { mem::transmute::<u64, [u8; 8]>(data.0) })
    }
}

impl Display for u8x8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:016X}", self.to_u64x1().0)
    }
}

impl u8x8 {
    pub fn new(data: [u8; 8]) -> Self {
        Self(data)
    }

    pub fn to_u64x1(&self) -> u64x1 {
        u64x1(unsafe { mem::transmute::<[u8; 8], u64>(self.0) })
    }

    pub fn overflowing_add(&self, x: &u8x8) -> u8x8 {
        u8x8(
            self.0.iter()
                .zip(x.0.iter())
                .map(|(x, y)| x.overflowing_add(*y).0)
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .expect("Overflowing addition - Conversion from [u8; X] to [u8; 8] failed.")
        )
    }

    pub fn saturating_add(&self, x: &Self) -> Self {
        Self(
            self.0.iter()
                .zip(x.0.iter())
                .map(|(x, y)| x.saturating_add(*y))
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .expect("Saturating addition - Conversion from [u8; X] to [u8; 8] failed.")
        )
    }

    pub fn overflowing_increment(&self) -> Self {
        Self(
            self.0.iter()
                .map(|x| x.overflowing_add(1).0)
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .expect("Increment - Conversion from [u8; X] to [u8; 8] failed.")
        )
    }

    pub fn saturating_increment(&self) -> Self {
        Self(
            self.0.iter()
                .map(|x| x.saturating_add(1))
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .expect("Increment - Conversion from [u8; X] to [u8; 8] failed.")
        )
    }

    pub fn multiply(&self, x: &Self) -> u16x4 {
        u16x4(
            self.0.iter()
                .zip(x.0.iter())
                .step_by(2)
                .map(|(x, y)| *x as u16 * *y as u16)
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .expect("Multiply - Conversion from [u16; X] to [u16; 4] failed.")
        )
    }
}

#[cfg(feature = "cli")]
use strum_macros::{EnumVariantNames, EnumString};

#[cfg(feature = "gui")]
use strum_macros::{EnumIter, Display};

#[cfg_attr(feature = "cli", derive(EnumString, EnumVariantNames))]
#[cfg_attr(feature = "gui", derive(EnumIter, Clone, PartialEq, Display))]
#[cfg_attr(any(feature = "debug", feature = "gui"), derive(Debug))]
pub enum Operations {
    PADDB,
    PADDSB,
    PINCB,
    PINCSB,
    PMULLB,

    PADDW,
    PADDSW,
    PINCW,
    PINCSW,
    PMULLW,

    PADDD,
    PADDSD,
    PINCD,
    PINCSD,
    PMULLD,

    PADDQ,
    PADDSQ,
    PINCQ,
    PINCSQ,
}

#[cfg(feature = "gui")]
impl Operations {
    pub fn requires_mm2(&self) -> bool {
        match self {
            Operations::PINCB | Operations::PINCSB | Operations::PINCW | Operations::PINCSW | Operations::PINCD | Operations::PINCSD | Operations::PINCQ | Operations::PINCSQ => false,
            _ => true
        }
    }
}