//! All axis parameters useable with TMCM modules other than TMCM-100 and Monopack 2.
//!
//! Mnemonics for macros:
//! - RLSD - RightLimitSwitchDisable
//! - LLSD - LeftLimitSwitchDisable

use AxisParameter;
use ReadableAxisParameter;
use WriteableAxisParameter;
use StorableAxisParameter;
use Return;

use modules::tmcm::{
    TmcmAxisParameter,
    ReadableTmcmAxisParameter,
    WriteableTmcmAxisParameter,
    StorableTmcmAxisParameter,
};


#[derive(Debug, PartialEq)]
pub struct RightLimitSwitchDisable {
    status: bool,
}
impl RightLimitSwitchDisable {
    pub fn disabled() -> Self {
        RightLimitSwitchDisable{status: true}
    }
    pub fn enabled() -> Self {
        RightLimitSwitchDisable{status: false}
    }
}
impl AxisParameter for RightLimitSwitchDisable {
    const NUMBER: u8 = 12;
}
impl Return for RightLimitSwitchDisable {
    fn deserialize(array: [u8; 4]) -> Self {RightLimitSwitchDisable{status: array[0] != 0}}
}
impl TmcmAxisParameter for RightLimitSwitchDisable {}
impl ReadableAxisParameter for RightLimitSwitchDisable {}
impl ReadableTmcmAxisParameter for RightLimitSwitchDisable {}
impl WriteableAxisParameter for RightLimitSwitchDisable {
    fn serialize_value(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, self.status as u8]
    }
}
impl WriteableTmcmAxisParameter for RightLimitSwitchDisable {}
impl StorableAxisParameter for RightLimitSwitchDisable {}
impl StorableTmcmAxisParameter for RightLimitSwitchDisable {}

#[derive(Debug, PartialEq)]
pub struct LeftLimitSwitchDisable {
    status: bool,
}
impl LeftLimitSwitchDisable {
    pub fn disabled() -> Self {
        LeftLimitSwitchDisable{status: true}
    }
    pub fn enabled() -> Self {
        LeftLimitSwitchDisable{status: false}
    }
}
impl AxisParameter for LeftLimitSwitchDisable {
    const NUMBER: u8 = 13;
}
impl Return for LeftLimitSwitchDisable {
    fn deserialize(array: [u8; 4]) -> Self {LeftLimitSwitchDisable{status: array[0] != 0}}
}
impl TmcmAxisParameter for LeftLimitSwitchDisable {}
impl ReadableAxisParameter for LeftLimitSwitchDisable {}
impl ReadableTmcmAxisParameter for LeftLimitSwitchDisable {}
impl WriteableAxisParameter for LeftLimitSwitchDisable {
    fn serialize_value(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, self.status as u8]
    }
}
impl WriteableTmcmAxisParameter for LeftLimitSwitchDisable {}
impl StorableAxisParameter for LeftLimitSwitchDisable {}
impl StorableTmcmAxisParameter for LeftLimitSwitchDisable {}