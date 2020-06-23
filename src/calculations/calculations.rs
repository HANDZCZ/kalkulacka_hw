use crate::types::*;

#[cfg_attr(feature = "debug", derive(Debug))]
pub enum EitherRegisters {
    OneRegister(u64x1),
    TwoRegisters(u64x1, u64x1),
}

pub fn calculate(mm1: u64x1, mm2: u64x1, operation: &Operations) -> EitherRegisters {
    match operation {
        Operations::PADDB | Operations::PADDSB | Operations::PINCB | Operations::PINCSB | Operations::PMULLB => {
            let mm1: u8x8 = mm1.into();
            let mm2: u8x8 = mm2.into();

            match operation {
                Operations::PADDB => mm1.overflowing_add(&mm2).into(),
                Operations::PADDSB => mm1.saturating_add(&mm2).into(),
                Operations::PINCB => (mm1.overflowing_increment(), mm2.overflowing_increment()).into(),
                Operations::PINCSB => (mm1.saturating_increment(), mm2.saturating_increment()).into(),
                Operations::PMULLB => mm1.multiply(&mm2).into(),
                _ => { unreachable!() }
            }
        }
        Operations::PADDW | Operations::PADDSW | Operations::PINCW | Operations::PINCSW | Operations::PMULLW => {
            let mm1: u16x4 = mm1.into();
            let mm2: u16x4 = mm2.into();

            match operation {
                Operations::PADDW => mm1.overflowing_add(&mm2).into(),
                Operations::PADDSW => mm1.saturating_add(&mm2).into(),
                Operations::PINCW => (mm1.overflowing_increment(), mm2.overflowing_increment()).into(),
                Operations::PINCSW => (mm1.saturating_increment(), mm2.saturating_increment()).into(),
                Operations::PMULLW => mm1.multiply(&mm2).into(),
                _ => { unreachable!() }
            }
        }
        Operations::PADDD | Operations::PADDSD | Operations::PINCD | Operations::PINCSD | Operations::PMULLD => {
            let mm1: u32x2 = mm1.into();
            let mm2: u32x2 = mm2.into();

            match operation {
                Operations::PADDD => mm1.overflowing_add(&mm2).into(),
                Operations::PADDSD => mm1.saturating_add(&mm2).into(),
                Operations::PINCD => (mm1.overflowing_increment(), mm2.overflowing_increment()).into(),
                Operations::PINCSD => (mm1.saturating_increment(), mm2.saturating_increment()).into(),
                Operations::PMULLD => mm1.multiply(&mm2).into(),
                _ => { unreachable!() }
            }
        }
        Operations::PADDQ => mm1.overflowing_add(&mm2).into(),
        Operations::PADDSQ => mm1.saturating_add(&mm2).into(),
        Operations::PINCQ | Operations::PINCSQ => {
            match operation {
                Operations::PINCQ => (mm1.overflowing_increment(), mm2.overflowing_increment()).into(),
                Operations::PINCSQ => (mm1.saturating_increment(), mm2.saturating_increment()).into(),
                _ => { unreachable!() }
            }
        }
    }
}

impl<T> From<T> for EitherRegisters where T: Into<u64x1> {
    fn from(data: T) -> Self {
        EitherRegisters::OneRegister(data.into())
    }
}

impl<T, Z> From<(T, Z)> for EitherRegisters where T: Into<u64x1>, Z: Into<u64x1> {
    fn from(data: (T, Z)) -> Self {
        EitherRegisters::TwoRegisters(data.0.into(), data.1.into())
    }
}