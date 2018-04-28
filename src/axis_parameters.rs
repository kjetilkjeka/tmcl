//! Axis parameters - useable with SAP, GAP, AAP, STAP and RSAP instructions.
//!
//! Please  note  that  the TMCM-100 units uses a different parameter set
//! (see TODO: insert link to module),
//! but all other TMCL stepper motor modules use these parameters

pub trait AxisParameter {
    const NUMBER: u8;

    fn serialize_value(&self) -> [u8; 4];
}

pub trait ReadableAxisParameter: AxisParameter {}

pub trait WriteableAxisParameter: AxisParameter {}

pub trait StorableAxisParameter: AxisParameter {}

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

    fn serialize_value(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, self.status as u8]
    }
}
impl ReadableAxisParameter for RightLimitSwitchDisable {}
impl WriteableAxisParameter for RightLimitSwitchDisable {}
impl StorableAxisParameter for RightLimitSwitchDisable {}

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

    fn serialize_value(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, self.status as u8]
    }
}
impl ReadableAxisParameter for LeftLimitSwitchDisable {}
impl WriteableAxisParameter for LeftLimitSwitchDisable {}
impl StorableAxisParameter for LeftLimitSwitchDisable {}