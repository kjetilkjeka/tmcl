//! TMCL - Trinamic Motion Control Language
//!
//! As described in [The TMCL Reference](https://www.mctechnology.nl/pdf/TMCL_reference_2015.pdf)

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "socketcan")]
extern crate socketcan;

#[cfg(feature = "socketcan")]
mod socketcan_impl;
mod command_macros;

mod instructions;
pub mod modules;

pub use instructions::Instruction;
use instructions::Return;

/// A interface for a TMCM module
///
/// Can be RS232, RS485, CAN or I2C
pub trait Interface {
    type Error;

    fn transmit_command<T: Instruction>(&self, command: &Command<T>) -> Result<(), Self::Error>;
    fn receive_reply(&self) -> Result<Reply, Self::Error>;
}

/// A `Comamnd` is an `Instruction` with a module address.
///
/// It contains everything required to serialize itself into Binary command format.
#[derive(Debug, PartialEq)]
pub struct Command<T: Instruction> {
    module_address: u8,
    instruction: T,
}

/// A TMCM module will respond with a `Reply` after receiving a `Command`.
#[derive(Debug, PartialEq, Clone)]
pub struct Reply {
    reply_address: u8,

    module_address: u8,

    status: Status,

    command_number: u8,

    value: [u8; 4],
}

/// Axis parameter - useable with SAP, GAP, AAP, STAP and/or RSAP instructions.
pub trait AxisParameter {
    /// The Parameter Number.
    const NUMBER: u8;
}

/// An axis parameter useable with the GAP instruction.
pub trait ReadableAxisParameter: AxisParameter + Return {}

/// An axis parameter useable with the SAP instruction.
pub trait WriteableAxisParameter: AxisParameter {
    fn serialize_value(&self) -> [u8; 4];
}

/// A `Status` that indicates that everything went well.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OkStatus {
    /// Successfully executed, no error
    Ok = 100,

    /// Command loaded into TMCL program EEPROM
    LoadedIntoEEPROM = 101,
}

/// A `Status` that indicate an `Error` has occured.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ErrStatus {
    /// Wrong checksum
    WrongChecksum = 1,

    /// Invalid command
    InvalidCommand = 2,

    /// Wrong type
    WrongType = 3,

    /// Invalid value
    InvalidValue = 4,

    /// Configuration EEPROM locked
    EEPROMLocked = 5,

    /// Command not available
    CommandNotAvailable = 6,
}

/// Every reply from a `Module` contains a `Status`
#[must_use]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Status {
    Ok(OkStatus),
    Err(ErrStatus),
}

impl<T: Instruction> Command<T> {
    pub fn new(module_address: u8, instruction: T) -> Command<T> {
        Command{module_address, instruction}
    }

    /// Returns the module address
    pub fn module_address(&self) -> u8 {
        self.module_address
    }

    /// Serialize into binary command format suited for RS232, RS485 etc
    ///
    /// The array will look like the following:
    /// `[MODULE_ADR, CMD_N, TYPE_N, MOTOR_N, VALUE3, VALUE2, VALUE1, VALUE0, CHECKSUM]`
    pub fn serialize(&self) -> [u8; 9] {
        unimplemented!()
    }

    /// Serialize into binary command format suited for I2C
    ///
    /// The array will look like the following:
    /// `[CMD_N, TYPE_N, MOTOR_N, VALUE3, VALUE2, VALUE1, VALUE0, CHECKSUM]`
    pub fn serialize_i2c(&self) -> [u8; 8] {
        unimplemented!()
    }

    /// Serialize into binary command format suited for CAN (controller area network)
    ///
    /// When using CAN the module address and checksum will be excluded.
    /// The array will look like the following:
    /// `[CMD_N, TYPE_N, MOTOR_N, VALUE3, VALUE2, VALUE1, VALUE0]`
    pub fn serialize_can(&self) -> [u8; 7] {
        [
            T::INSTRUCTION_NUMBER,
            self.instruction.type_number(),
            self.instruction.motor_number(),
            self.instruction.serialize_value()[0],
            self.instruction.serialize_value()[1],
            self.instruction.serialize_value()[2],
            self.instruction.serialize_value()[3],
        ]
    }

}

impl Reply {
    fn new(
        reply_address: u8,
        module_address: u8,
        status: Status,
        command_number: u8,
        value: [u8; 4],
    ) -> Self {
        Reply {
            reply_address,
            module_address,
            status,
            command_number,
            value,
        }
    }

    fn value(&self) -> [u8; 4] {
        self.value
    }

    fn status(&self) -> Status {
        self.status
    }
}

impl Status {
    /// Fallible conversion from `u8`
    fn try_from_u8(id: u8) -> Result<Status, NonValidErrorCode> {
        match id {
            100 => Ok(Status::Ok(OkStatus::Ok)),
            101 => Ok(Status::Ok(OkStatus::LoadedIntoEEPROM)),
            1 => Ok(Status::Err(ErrStatus::WrongChecksum)),
            2 => Ok(Status::Err(ErrStatus::InvalidCommand)),
            3 => Ok(Status::Err(ErrStatus::WrongType)),
            4 => Ok(Status::Err(ErrStatus::InvalidValue)),
            5 => Ok(Status::Err(ErrStatus::EEPROMLocked)),
            6 => Ok(Status::Err(ErrStatus::CommandNotAvailable)),
            _ => Err(NonValidErrorCode),
        }
    }
}

/// The result of attempting to converted a number that is not a valid status code into `Status`.
#[derive(Debug)]
pub struct NonValidErrorCode;

impl Return for () {
    fn deserialize(_array: [u8; 4]) -> () {()}
}

impl Return for bool {
    fn deserialize(array: [u8; 4]) -> bool {(array[3] & 1) != 0}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
