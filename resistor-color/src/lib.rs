use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;
use std::fmt;

// #[derive(Debug, PartialEq)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    Black=0,
    Brown=1,
    Red=2,
    Orange=3,
    Yellow=4,
    Green=5,
    Blue=6,
    Violet=7,
    Grey=8,
    White=9,
}

impl fmt::Display for ResistorColor{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{
        match *self{
            ResistorColor::Black => write!(f, "Black"),
            ResistorColor::Brown => write!(f, "Brown"),
            ResistorColor::Red => write!(f, "Red"),
            ResistorColor::Orange => write!(f, "Orange"),
            ResistorColor::Yellow => write!(f, "Yellow"),
            ResistorColor::Green => write!(f, "Green"),
            ResistorColor::Blue => write!(f, "Blue"),
            ResistorColor::Violet => write!(f, "Violet"),
            ResistorColor::Grey => write!(f, "Grey"),
            ResistorColor::White => write!(f, "White"),
        }
    }
}

pub fn color_to_value(color: ResistorColor) -> u8 {
    color.int_value()

}

pub fn value_to_color_string(value: u8) -> String {
    let e = ResistorColor::from_int(value);
    match e {
        Ok(e) => e.to_string(),
        Err(_)=> String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect::<Vec<ResistorColor>>()
}
