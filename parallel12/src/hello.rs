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

impl Future for Hello {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        match (*self).state {
            StateHello::HELLO => {
                print!("Hello, ");
                (*self).state = StateHello::WORLD;
                Poll::Pending
            }
            StateHello::WORLD => {
                println!("World!");
                (*self).state = StateHello::END;
                Poll::Pending
            }
            StateHello::END => Poll::Ready(()),
        }
    }
}
