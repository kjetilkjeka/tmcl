//! `Instruction`s available for the generic TMCM module.

use instructions::Instruction;
use instructions::DirectInstruction;

pub use instructions::{
    ROR,
    ROL,
    MST,
    MVP,
    RFS,
    SIO,
    GIO,
    Move,
    ReferenceSearchAction,
};

/// SAP - Set Axis Parameter
///
/// Most parameters of a TMCM module can be adjusted individually for each axis.
/// Although  these parameters vary widely in their formats (1 to 24 bits, signed or unsigned)
/// and physical locations (TMC428, TMC453, controller RAM, controller EEPROM),
/// they all can be set by this function.
#[derive(Debug, PartialEq)]
pub struct SAP {
    motor_number: u8,
    parameter_number: u8,
    operand: [u8; 4],
}
impl SAP {
    pub fn new(motor_number: u8, parameter_number: u8, operand: [u8; 4]) -> SAP {
        SAP{
            motor_number,
            parameter_number,
            operand,
        }
    }
}
impl Instruction for SAP {
    const INSTRUCTION_NUMBER: u8 = 5;

    fn serialize_value(&self) -> [u8; 4] {
        [self.operand[3], self.operand[2], self.operand[1], self.operand[0]]
    }

    fn type_number(&self) -> u8 {
        self.parameter_number
    }

    fn motor_bank_number(&self) -> u8 {
        self.motor_number
    }
}
impl DirectInstruction for SAP {
    type Return = ();
}

/// GAP - Get Axis Parameter
///
/// Most parameters of a TMCM module can be adjusted individually for each axis.
/// Although  these parameters vary widely in their formats (1 to 24 bits, signed or unsigned)
/// and physical locations (TMC428, TMC453, controller RAM, controller EEPROM),
/// they all can be read by this function.
#[derive(Debug, PartialEq)]
pub struct GAP {
    motor_number: u8,
    parameter_number: u8,
}
impl GAP {
    pub fn new(motor_number: u8, parameter_number: u8) -> GAP {
        GAP{
            motor_number,
            parameter_number,
        }
    }
}
impl Instruction for GAP {
    const INSTRUCTION_NUMBER: u8 = 6;

    fn serialize_value(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, 0u8]
    }

    fn type_number(&self) -> u8 {
        self.parameter_number
    }

    fn motor_bank_number(&self) -> u8 {
        self.motor_number
    }
}
impl DirectInstruction for GAP {
    type Return = [u8; 4];
}

/// STAP - Store Axis Parameter
///
/// Axis parameters are located in RAM memory, so modifications are lost at power down.
/// This instruction enables permanent storing.
#[derive(Debug, PartialEq)]
pub struct STAP {
    motor_number: u8,
    parameter_number: u8,
}
impl STAP {
    pub fn new(motor_number: u8, parameter_number: u8) -> STAP {
        STAP{
            motor_number,
            parameter_number,
        }
    }
}
impl Instruction for STAP {
    const INSTRUCTION_NUMBER: u8 = 7;

    fn serialize_value(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, 0u8]
    }

    fn type_number(&self) -> u8 {
        self.parameter_number
    }

    fn motor_bank_number(&self) -> u8 {
        self.motor_number
    }
}
impl DirectInstruction for STAP {
    type Return = ();
}