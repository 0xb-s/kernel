use core::{any::Any, fmt::Debug};

use super::{input::Key, KeyStatus};
#[derive(Debug, Clone, Copy)]

pub enum InputEvent {
    KeyBoard(Key, KeyStatus),
}
pub trait InputDevice: Send + Sync + Any + Debug {
    type CallBack;

    fn get_call_back(&self, call_back: Self::CallBack);
}
