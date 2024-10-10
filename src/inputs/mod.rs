pub mod events;
pub mod input;

#[derive(Debug, Clone, Copy)]

pub enum SupportedKey {
    English,
}

#[derive(Debug, Clone, Copy)]

pub enum KeyStatus {
    Done,
    Pressed,
}
