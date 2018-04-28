#![no_std]

pub mod instructions;
pub mod axis_parameters;

pub use axis_parameters::{
    AxisParameter,
    ReadableAxisParameter,
    WriteableAxisParameter,
    StorableAxisParameter,
};

pub use instructions::Instruction;

/// A `Comamnd` is an `Instruction` with a module address.
///
/// It contains everything required to serialize itself into Binary command format.
#[derive(Debug, PartialEq)]
pub struct Command<T: Instruction> {
    module_address: u8,
    instruction: T,
}

#[derive(Debug, PartialEq)]
pub struct Reply {
    // TODO: Add fields
    status: Status,

    command_number: u8,
}

#[derive(Debug, PartialEq)]
pub enum Status {

    /// Successfully executed, no error
    Ok = 100,

    /// Command loaded into TMCL program EEPROM
    LoadedIntoEEPROM = 101,

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
