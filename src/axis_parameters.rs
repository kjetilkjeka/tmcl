//! Tools for implementing axis parameters.
//!
//! ## `axis_param` macros
//! These macros works for most cases, if the register is represented by an enum or
//! a type not implemented `Return` for these macros wont work.

macro_rules! axis_param_r {
    ($(#[$doc:meta])* $name:ident, $ty:ty, $number:expr) => {
        axis_param_define!($(#[$doc])* $name, $ty, $number);
        axis_param_define_read!($name, $ty);
    };
}
/*
macro_rules! axis_param_w {
    ($(#[$doc:meta])* $name:ident, $ty:ty, $number:expr) => {
        axis_param_define!($(#[$doc])* $name, $ty, $number);
        axis_param_define_write!($name, $ty);
    };
}
*/
macro_rules! axis_param_rw {
    ($(#[$doc:meta])* $name:ident, $ty:tt, $number:expr) => {
        axis_param_define!($(#[$doc])* $name, $ty, $number);
        axis_param_define_read!($name, $ty);
        axis_param_define_write!($name, $ty);
    };
}

macro_rules! axis_param_define{
    ($(#[$doc:meta])* $name:ident, $ty:ty, $number:expr) => {
        $(#[$doc])*
        #[derive(Debug, PartialEq)]
        pub struct $name($ty);

        impl From<$name> for $ty {
            fn from(v: $name) -> $ty {
                v.0
            }
        }

        impl AxisParameter for $name {
            const NUMBER: u8 = $number;
        }
    };
}

macro_rules! axis_param_define_read {
    ($name:ident, $ty:ty) => {
        impl Return for $name {
            fn from_operand(operand: [u8; 4]) -> Self {
                $name(<$ty as Return>::from_operand(operand))
            }
        }
        impl ReadableAxisParameter for $name {}
    };
}

macro_rules! axis_param_define_write {
    ($name:ident, u32) => {
        impl WriteableAxisParameter for $name {
            fn operand(&self) -> [u8; 4] {
                [(self.0 >> 0) as u8, (self.0 >> 8) as u8, (self.0 >> 16) as u8 , (self.0 >> 24) as u8]
            }
        }
    };
    ($name:ident, u16) => {
        impl WriteableAxisParameter for $name {
            fn operand(&self) -> [u8; 4] {
                [(self.0 >> 0) as u8, (self.0 >> 8) as u8, 0u8 , 0u8]
            }
        }
    };
    ($name:ident, u8) => {
        impl WriteableAxisParameter for $name {
            fn operand(&self) -> [u8; 4] {
                [(self.0 >> 0) as u8, 0u8, 0u8 , 0u8]
            }
        }
    };
    ($name:ident, i32) => {
        impl WriteableAxisParameter for $name {
            fn operand(&self) -> [u8; 4] {
                [(self.0 >> 0) as u8, (self.0 >> 8) as u8, (self.0 >> 16) as u8 , (self.0 >> 24) as u8]
            }
        }
    };
    ($name:ident, i16) => {
        impl WriteableAxisParameter for $name {
            fn operand(&self) -> [u8; 4] {
                [(self.0 >> 0) as u8, (self.0 >> 8) as u8, 0u8, 0u8]
            }
        }
    };
    ($name:ident, i8) => {
        impl WriteableAxisParameter for $name {
            fn operand(&self) -> [u8; 4] {
                [(self.0 >> 0) as u8, 0u8, 0u8, 0u8]
            }
        }
    };
    ($name:ident, bool) => {
        impl WriteableAxisParameter for $name {
            fn operand(&self) -> [u8; 4] {
                [self.0 as u8, 0, 0, 0]
            }
        }
    };
}