use crossbeam::channel;
use futures::task::{self, ArcWake};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};
struct MiniTokio {
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task>>,
}

impl Task {
    fn poll(self: Arc<Self>) {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        let mut future = self.future.try_lock().unwrap();
        let _ = future.as_mut().poll(&mut cx);
    }

    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }

    fn schedule(self: &Arc<Self>) {
        self.executor.send(self.clone());
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}
impl MiniTokio {
    fn new() -> Self {
        let (sender, scheduled) = channel::unbounded();
        Self { scheduled, sender }
    }

    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }

    fn run(&mut self) {
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Hello, World");
            Poll::Ready("done")
        } else {
            // 現在のタスクに紐づく"waker"ハンドルを取得
            let waker = cx.waker().clone();
            let when = self.when;

            // タイマースレッドをspawn
            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now);
                    waker.wake();
                }
            });
            Poll::Pending
        }
    }
}

fn main() {
    let mut mini_tokio = MiniTokio::new();

    for i in 1..=10 {
        mini_tokio.spawn(async move {
            let when = Instant::now() + Duration::from_micros(10000000);
            let future = Delay { when };

            println!("{}", i);

            let out = future.await;
            assert_eq!(out, "done");
        });
    }

    mini_tokio.run();
}
