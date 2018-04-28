use std::io;

use socketcan::{
    CANSocket,
    CANFrame,
};

use Interface;
use Instruction;
use Reply;
use Command;

impl Interface for CANSocket {
    type Error = io::Error;

    fn transmit_command<T: Instruction>(&self, command: &Command<T>) -> Result<(), Self::Error> {
        let frame = CANFrame::new(u32::from(command.module_address), &command.serialize_can(), false, false).unwrap();
        self.write_frame_insist(&frame)
    }

    fn receive_reply(&self) -> Result<Reply, Self::Error> {
        //let frame = self.read_frame()?;
        unimplemented!()
    }
}