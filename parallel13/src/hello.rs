use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
pub struct Hello {
    pub state: StateHello,
}

pub enum StateHello {
    HELLO,
    WORLD,
    END,
}

impl Hello {
    pub fn new() -> Self {
        Self {
            state: StateHello::HELLO,
        }
    }
}
