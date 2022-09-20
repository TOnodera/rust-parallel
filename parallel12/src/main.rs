mod hello;
use futures::future::{BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

struct Task {
    hello: Mutex<BoxFuture<'static, ()>>,
}

impl Task {
    fn new() -> Self {
        let hello = hello::Hello::new();
        Self {
            hello: Mutex::new(hello.boxed()),
        }
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {}
}

fn main() {
    let task = Arc::new(Task::new());
    let waker = waker_ref(&task);
    let mut ctx = Context::from_waker(&waker);
    let mut hello = task.hello.lock().unwrap();

    hello.as_mut().poll(&mut ctx);
    hello.as_mut().poll(&mut ctx);
    hello.as_mut().poll(&mut ctx);
}
