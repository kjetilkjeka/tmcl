//! TMCL - Trinamic Motion Control Language
//!
//! As described in [The TMCL Reference](https://www.mctechnology.nl/pdf/TMCL_reference_2015.pdf)
//!
//! # Features
//! ## Read/Write register safety
//! When using a specific module an attempt to write to a read only register will fail to compile.
//!
//! ```compile_fail
//! extern crate tmcl;
//!
//! use std::cell::RefCell;
//! use tmcl::modules::generic::instructions::*;
//! use tmcl::modules::tmcm::axis_parameters::*;
//! // Here we use a specific module
//! use tmcl::modules::tmcm::TmcmModule as Module;
//!
//! # use tmcl::Interface;
//! # use tmcl::Instruction;
//! # use tmcl::Command;
//! # use tmcl::Reply;
//! #
//! # struct MyInterface();
//! # #[derive(Debug)]
//! # struct MyInterfaceError();
//! #
//! # impl MyInterface { fn new() -> Self {unimplemented!()} }
//! #
//! # impl Interface for MyInterface {
//!    # type Error = MyInterfaceError;
//!    # fn transmit_command<T: Instruction>(&mut self, command: &Command<T>) -> Result<(), Self::Error> {
//!        # unimplemented!()
//!    # }
//!    # fn receive_reply(&mut self) -> Result<Reply, Self::Error> {
//!        # unimplemented!()
//!    # }
//! # }
//! #
//! fn main() -> Result<(), tmcl::Error<MyInterfaceError>> {
//!     let interface = RefCell::new(MyInterface::new());
//!
//!     let module = Module::new(&interface, 1);
//!
//!     // Since ActualSpeed is a read only register,
//!     // reading it is the only way construct the type.
//!     let actual_speed = module.write_command(GAP::<ActualSpeed>::new(0))?;
//!     module.write_command(SAP::new(0, actual_speed))?;
//!
//!     Ok(())
//! }
//! ```
//!
//! These guarantees do not hold with the generic module as it doesn't know anything about register types.
//! The following code will fail to write `ActualSpeed` in runtime instead of compile-time.
//!
//! ```no_run
//! extern crate tmcl;
//!
//! use std::cell::RefCell;
//! use tmcl::modules::generic::instructions::*;
//! // We use the Generic module instead
//! use tmcl::modules::generic::GenericModule;
//!
//! # use tmcl::Interface;
//! # use tmcl::Instruction;
//! # use tmcl::Command;
//! # use tmcl::Reply;
//! #
//! # struct MyInterface();
//! # #[derive(Debug)]
//! # struct MyInterfaceError();
//! #
//! # impl MyInterface { fn new() -> Self {unimplemented!()} }
//! #
//! # impl Interface for MyInterface {
//!    # type Error = MyInterfaceError;
//!    # fn transmit_command<T: Instruction>(&mut self, command: &Command<T>) -> Result<(), Self::Error> {
//!        # unimplemented!()
//!    # }
//!    # fn receive_reply(&mut self) -> Result<Reply, Self::Error> {
//!        # unimplemented!()
//!    # }
//! # }
//! #
//! fn main() {
//!     let interface = RefCell::new(MyInterface::new());
//!
//!     let module = GenericModule::new(&interface, 1);
//!
//!     assert_eq!(
//!         module.write_command(SAP::new(0, 3, [0u8, 0u8, 0u8, 0u8])),
//!         Error::ProtocolError(ErrStatus::WrongType)
//!     );
//! }
//! ```
//!
//! # Examples
//! ## Socketcan
//! To use this example the socketcan feature must be enabled.
//! And a socketcan interface named `vcan0` must exist.
//!
//! ```no_run
//! extern crate tmcl;
//! # #[cfg(all(feature = "std", feature = "socketcan"))]
//! extern crate socketcan;
//!
//! # #[cfg(all(feature = "std", feature = "socketcan"))]
//! use std::cell::RefCell;
//!
//! use tmcl::modules::tmcm::instructions::*;
//! use tmcl::modules::tmcm::axis_parameters::*;
//! use tmcl::modules::tmcm::TmcmModule as Module;
//! # #[cfg(all(feature = "std", feature = "socketcan"))]
//! fn main() {
//!     # std::process::Command::new("sudo ip link add dev vcan0 type vcan").output();
//!     # std::process::Command::new("sudo ip link set up vcan0").output();
//!     let interface = RefCell::new(socketcan::CANSocket::open("vcan0").unwrap());
//!
//!     let module1 = Module::new(&interface, 1);
//!     let module2 = Module::new(&interface, 2);
//!
//!     module1.write_command(ROR::new(0, 250)).unwrap();
//!     module2.write_command(ROL::new(0, 250)).unwrap();
//! }
//! # #[cfg(not(all(feature = "std", feature = "socketcan")))]
//! # fn main() {}
//! ```
//!
//! ## Socketcan and threading
//! To use this example the socketcan feature must be enabled.
//! And a socketcan interface named `vcan0` must exist.
//!
//! ```no_run
//! extern crate tmcl;
//! # #[cfg(all(feature = "std", feature = "socketcan"))]
//! extern crate socketcan;
//!
//! use std::sync::Mutex;
//! use std::sync::Arc;
//!
//! use tmcl::modules::tmcm::instructions::*;
//! use tmcl::modules::tmcm::axis_parameters::*;
//! use tmcl::modules::tmcm::TmcmModule as Module;
//!
//! # #[cfg(all(feature = "std", feature = "socketcan"))]
//! fn main() {
//!     # std::process::Command::new("sudo ip link add dev vcan0 type vcan").output();
//!     # std::process::Command::new("sudo ip link set up vcan0").output();
//!
//!     let interface = Arc::new(Mutex::new(socketcan::CANSocket::open("vcan0").unwrap()));
//!
//!     let module1 = Module::new(interface.clone(), 1);
//!     let module2 = Module::new(interface, 2);
//!
//!     std::thread::spawn(move || {
//!         module1.write_command(ROR::new(0, 250)).unwrap();
//!     });
//!
//!     std::thread::spawn(move || {
//!         module2.write_command(ROL::new(0, 250)).unwrap();
//!     });
//! }
//! # #[cfg(not(all(feature = "std", feature = "socketcan")))]
//! # fn main() {}
//! ```
//!
//! ## No-std
//! When using with no-std you can implement `Interface` on the interface you intent to use.
//!
//! ```ignore
//! # // TODO: change ignore to no_run once panic_implementation is stabilized
//! #![no_std]
//!
//! extern crate tmcl;
//!
//! use core::cell::RefCell;
//!
//! use tmcl::Interface;
//! use tmcl::Reply;
//! use tmcl::Command;
//! use tmcl::Instruction;
//! use tmcl::modules::tmcm::instructions::*;
//! use tmcl::modules::tmcm::axis_parameters::*;
//! use tmcl::modules::tmcm::TmcmModule as Module;
//!
//! # struct MyInterface();
//! # struct MyInterfaceError();
//!
//! # impl MyInterface { fn new() -> Self {unimplemented!()} }
//!
//! impl Interface for MyInterface {
//!    type Error = MyInterfaceError;
//!
//!    fn transmit_command<T: Instruction>(&mut self, command: &Command<T>) -> Result<(), Self::Error> {
//!        // Implement transmit_command for your interface
//!        # unimplemented!()
//!    }
//!
//!    fn receive_reply(&mut self) -> Result<Reply, Self::Error> {
//!        // Implement receive_reply for your interface
//!        # unimplemented!()
//!    }
//! }
//!
//! fn main() {
//!
//!     let interface = RefCell::new(MyInterface::new());
//!
//!     let module1 = Module::new(&interface, 1);
//!     let module2 = Module::new(&interface, 2);
//!
//!     module1.write_command(ROR::new(0, 250)).unwrap();
//!     module2.write_command(ROL::new(0, 250)).unwrap();
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

mod lib {
    #[cfg(feature = "std")]
    pub use std::*;
    #[cfg(not(feature = "std"))]
    pub use core::*;
}

extern crate interior_mut;

#[cfg(feature = "socketcan")]
extern crate socketcan;

#[cfg(feature = "socketcan")]
mod socketcan_impl;

mod instructions;
#[macro_use]
mod axis_parameters;

pub mod modules;

pub use instructions::Instruction;
use instructions::Return;

/// A interface for a TMCM module
///
/// Can be RS232, RS485, CAN or I2C
pub trait Interface {
    type Error;

    fn transmit_command<T: Instruction>(&mut self, command: &Command<T>) -> Result<(), Self::Error>;
    fn receive_reply(&mut self) -> Result<Reply, Self::Error>;
}

/// All possible errors when communicating with
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Error<T> {

    /// This means that the library was not able to get the mutable reference to the interface.
    ///
    /// This can be cause by many different things
    ///  - If `RefCell` is used then the interface might be used by a different stepper motor.
    ///  - If `Mutex` is used a thread may have panicked and the mutex is poisoned.
    InterfaceUnavailable,

    /// The interface had an error.
    InterfaceError(T),

    /// The `TMCL` module reported an error.
    ProtocolError(ErrStatus),
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

    operand: [u8; 4],
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
    fn operand(&self) -> [u8; 4];
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
            self.instruction.motor_bank_number(),
            self.instruction.operand()[3],
            self.instruction.operand()[2],
            self.instruction.operand()[1],
            self.instruction.operand()[0],
        ]
    }

}

impl Reply {
    pub fn new(
        reply_address: u8,
        module_address: u8,
        status: Status,
        command_number: u8,
        operand: [u8; 4],
    ) -> Self {
        Reply {
            reply_address,
            module_address,
            status,
            command_number,
            operand,
        }
    }

    fn operand(&self) -> [u8; 4] {
        self.operand
    }

    fn status(&self) -> Status {
        self.status
    }
}

impl Status {
    /// Fallible conversion from `u8`
    pub fn try_from_u8(id: u8) -> Result<Status, NonValidErrorCode> {
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
    fn from_operand(_operand: [u8; 4]) -> () {()}
}

impl Return for [u8; 4] {
    fn from_operand(array: [u8; 4]) -> [u8; 4] {
        array
    }
}

impl Return for bool {
    fn from_operand(array: [u8; 4]) -> bool {
        array[0] != 0 || array[1] != 0 || array[2] != 0 || array[3] != 0
    }
}

impl Return for i32 {
    fn from_operand(array: [u8; 4]) -> i32 {
        (array[0] as u32 | ((array[1] as u32) << 8) |  ((array[2] as u32) << 16) |((array[3] as u32) << 24)) as i32
    }
}

impl Return for i16 {
    fn from_operand(array: [u8; 4]) -> i16 {
        (array[0] as u16 | ((array[1] as u16) << 8)) as i16
    }
}

impl Return for i8 {
    fn from_operand(array: [u8; 4]) -> i8 {
        array[0] as i8
    }
}

impl Return for u32 {
    fn from_operand(array: [u8; 4]) -> u32 {
        (array[0] as u32 | ((array[1] as u32) << 8) |  ((array[2] as u32) << 16) |((array[3] as u32) << 24))
    }
}

impl Return for u16 {
    fn from_operand(array: [u8; 4]) -> u16 {
        array[0] as u16 | ((array[1] as u16) << 8)
    }
}

impl Return for u8 {
    fn from_operand(array: [u8; 4]) -> u8 {
        array[0]
    }
}

impl<T> From<ErrStatus> for Error<T> {
    fn from(es: ErrStatus) -> Self {
        Error::ProtocolError(es)
    }
}
