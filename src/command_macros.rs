#[macro_export]
macro_rules! tmcl_instruction {
    (ROR $motor_number:expr, $velocity:expr) => {ROR::new($motor_number, $velocity)};
    (ROL $motor_number:expr, $velocity:expr) => {ROL::new($motor_number, $velocity)};
    (MST $motor_number:expr) => {MST::new($motor_number)};

    (MVP ABS, $motor_number:expr, $value:expr) => {MVP::new($motor_number, Move::Absolute($value))};
    (MVP REL, $motor_number:expr, $value:expr) => {MVP::new($motor_number, Move::Relative($value))};

    // expansions of all possible coordinates
    (MVP COORD, $motor_number:expr, 0) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate0))};
    (MVP COORD, $motor_number:expr, 1) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate1))};
    (MVP COORD, $motor_number:expr, 2) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate2))};
    (MVP COORD, $motor_number:expr, 3) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate3))};
    (MVP COORD, $motor_number:expr, 4) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate4))};
    (MVP COORD, $motor_number:expr, 5) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate5))};
    (MVP COORD, $motor_number:expr, 6) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate6))};
    (MVP COORD, $motor_number:expr, 7) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate7))};
    (MVP COORD, $motor_number:expr, 8) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate8))};
    (MVP COORD, $motor_number:expr, 9) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate9))};
    (MVP COORD, $motor_number:expr, 10) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate10))};
    (MVP COORD, $motor_number:expr, 11) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate11))};
    (MVP COORD, $motor_number:expr, 12) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate12))};
    (MVP COORD, $motor_number:expr, 13) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate13))};
    (MVP COORD, $motor_number:expr, 14) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate14))};
    (MVP COORD, $motor_number:expr, 15) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate15))};
    (MVP COORD, $motor_number:expr, 16) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate16))};
    (MVP COORD, $motor_number:expr, 17) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate17))};
    (MVP COORD, $motor_number:expr, 18) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate18))};
    (MVP COORD, $motor_number:expr, 19) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate19))};
    (MVP COORD, $motor_number:expr, 20) => {MVP::new($motor_number, Move::Coordinate(Coordinate::Coordinate20))};

    // SAP instructions with tmcm mnemonics
    (SAP MPS, $motor_number:expr, $speed:expr) => {SAP::new($motor_number, MaximumPositioningSpeed::new($speed))};

    (SAP RLSD, $motor_number:expr, 1) => {SAP::new($motor_number, RightLimitSwitchDisable::disabled())};
    (SAP RLSD, $motor_number:expr, 0) => {SAP::new($motor_number, RightLimitSwitchDisable::enabled())};

    (SAP LLSD, $motor_number:expr, 1) => {SAP::new($motor_number, LeftLimitSwitchDisable::disabled())};
    (SAP LLSD, $motor_number:expr, 0) => {SAP::new($motor_number, LeftLimitSwitchDisable::enabled())};

    // GAP instructions with tmcm mnemonics
    (GAP RLSD, $motor_number:expr) => {GAP::<RightLimitSwitchDisable>::new($motor_number)};
    (GAP LLSD, $motor_number:expr) => {GAP::<LeftLimitSwitchDisable>::new::($motor_number)};
    (GAP AS, $motor_number:expr) => {GAP::<ActualSpeed>::new($motor_number)};
    (GAP AP, $motor_number:expr) => {GAP::<ActualPosition>::new($motor_number)};

    // STAP instructions with tmcm mnemonics
    (STAP RLSD, $motor_number:expr) => {STAP::<RightLimitSwitchDisable>::new($motor_number)};
    (STAP LLSD, $motor_number:expr) => {STAP::<LeftLimitSwitchDisable>::new($motor_number)};
    (STAP AS, $motor_number:expr) => {STAP::<ActualSpeed>::new($motor_number)};
    (STAP AP, $motor_number:expr) => {STAP::<ActualPosition>::new($motor_number)};

    // RFS instruction
    (RFS START, $motor_number:expr) => {RFS::new($motor_number, ReferenceSearchAction::Start)};
    (RFS STOP, $motor_number:expr) => {RFS::new($motor_number, ReferenceSearchAction::Stop)};
    (RFS STATUS, $motor_number:expr) => {RFS::new($motor_number, ReferenceSearchAction::Status)};
}
