struct Hello {
    state: StateHello,
}

enum StateHello {
    HELLO,
    WORLD,
    END,
}

impl Hello {
    fn new() -> Self {
        Self {
            state: StateHello::HELLO,
        }
    }
}

impl Future for Hello {}
fn main() {
    println!("Hello, world!");
}
