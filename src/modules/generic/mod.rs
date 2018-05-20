//! Generic `TMCM` implementation - uses `TMCL` instructions without making assumptions about parameters.
//!
//! The types in module only provides a minimum of compile time guarantees.
//! It is therefore preferable to use a less generic module that will fail to compile if
//! it is attempted to write to a read only register and such.
//! This module is only recommended to use if no such module exists.

pub mod instructions;

use lib::ops::Deref;
use lib::marker::PhantomData;

use interior_mut::InteriorMut;

use Error;
use Instruction;
use instructions::DirectInstruction;
use Interface;
use Return;
use Status;
use Command;

/// This type represents a generic TMCM module.
#[derive(Debug)]
pub struct GenericModule<'a, IF: Interface + 'a, Cell: InteriorMut<'a, IF>, T: Deref<Target=Cell> + 'a> {
    /// The module address
    address: u8,
    interface: T,
    pd1: PhantomData<&'a IF>,
    pd2: PhantomData<&'a T>,
}

impl<'a, IF: Interface, Cell: InteriorMut<'a, IF>, T: Deref<Target=Cell>> GenericModule<'a, IF, Cell, T> {
    /// Create a new module
    pub fn new(interface: T, address: u8) -> Self {
        GenericModule{
            address,
            interface,
            pd1: PhantomData{},
            pd2: PhantomData{},
        }
    }

    /// Synchronously write a command and wait for the Reply
    pub fn write_command<Inst: Instruction + DirectInstruction>(&'a self, instruction: Inst) -> Result<Inst::Return, Error<IF::Error>> {
        let mut interface = self.interface.borrow_int_mut().or(Err(Error::InterfaceUnavailable))?;
        interface.transmit_command(&Command::new(self.address, instruction)).map_err(|e| Error::InterfaceError(e))?;
        let reply = interface.receive_reply().map_err(|e| Error::InterfaceError(e))?;
        match reply.status() {
            Status::Ok(_) => Ok(<Inst::Return as Return>::from_operand(reply.operand())),
            Status::Err(e) => Err(e.into()),
        }
    }
}