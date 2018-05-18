use std::io;

use socketcan::{
    CANSocket,
    CANFrame,
};

use Interface;
use Instruction;
use Reply;
use Command;
use Status;

impl Interface for CANSocket {
    type Error = io::Error;

    fn transmit_command<T: Instruction>(&mut self, command: &Command<T>) -> Result<(), Self::Error> {
        let frame = CANFrame::new(u32::from(command.module_address), &command.serialize_can(), false, false).unwrap();
        self.write_frame_insist(&frame)
    }

    fn receive_reply(&mut self) -> Result<Reply, Self::Error> {
        // TODO: make robust
        let frame = self.read_frame()?;
        Ok(Reply::new(
            frame.id() as u8,
            frame.data()[0],
            Status::try_from_u8(frame.data()[1]).unwrap(),
            frame.data()[2],
            [frame.data()[3], frame.data()[4], frame.data()[5], frame.data()[6]],
        ))
    }
}