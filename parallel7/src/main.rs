mod banker;

use banker::Banker;
use std::thread;

const NUM_LOOP: usize = 100000;
fn main() {
    let banker = Banker::<2, 2>::new([1, 1], [[1, 1], [1, 1]]);
    let banker0 = banker.clone();

    let p0 = thread::spawn(move || {
        for _ in 0..NUM_LOOP {
            while !banker0.take(0, 0) {}
            while !banker0.take(0, 1) {}

            println!("0: eating");

            banker0.release(0, 0);
            banker0.release(0, 1);
        }
    });

    let p1 = thread::spawn(move || {
        for _ in 0..NUM_LOOP {
            while !banker.take(1, 1) {}
            while !banker.take(1, 0) {}

            println!("1: eating");

            banker.release(1, 1);
            banker.release(1, 0);
        }
    });

    p0.join().unwrap();
    p1.join().unwrap();
}
