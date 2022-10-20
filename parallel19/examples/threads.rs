use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

fn main() {
    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    let mut is_running = true;

    let th1 = thread::spawn(move || {
        while true {
            thread::sleep(Duration::from_secs(1));
            match rx.try_recv() {
                Ok(running) => {
                    println!("capture...");
                    if (!running) {
                        break;
                    }
                }
                Err(_) => {
                    println!("none...");
                    continue;
                }
            }
        }
    });

    let th2 = thread::spawn(move || {
        let mut cnt = 0;
        while true {
            thread::sleep(Duration::from_secs(1));
            println!("送信側カウント中...");
            if cnt > 10 {
                tx.send(false).unwrap();
                break;
            }
            cnt += 1;
        }
    });

    th1.join().unwrap();
    th2.join().unwrap();
}
