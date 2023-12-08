use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
struct App {
    systems: Vec<*mut (dyn Fn(i32, Sender<i32>) + Sync + Send + 'static)>,
    main_channel: (Sender<i32>, Receiver<i32>),
}
impl App {
    fn new() -> Self {
        App {
            systems: Vec::new(),
            main_channel: mpsc::channel(),
        }
    }
    fn add_system<T>(&mut self, f: T) -> &mut Self
    where
        T: Fn(i32, Sender<i32>) + Sync + Send + 'static,
    {
        self.systems.push(Box::into_raw(Box::new(f)));
        self
    }
    fn run(&self) {
        self.main_channel.0.clone().send(0).unwrap();
        for v in &self.main_channel.1 {
            for f in &self.systems {
                let sender = self.main_channel.0.clone();
                let f = unsafe { Box::from_raw(f.clone()) };
                thread::spawn(move || f(v, sender));
            }
        }
    }
}
fn main() {
    let mut app = App::new();
    app.add_system(hello_world).add_system(hello_app).run();
}
fn hello_world(a: i32, s: Sender<i32>) {
    println!("hello world, {}", a);
    s.send(a + 1).unwrap();
}
fn hello_app(a: i32, _: Sender<i32>) {
    println!("hello app, {}", a)
}
