mod app;
mod app_mpsc;

use std::thread;
use std::time::Duration;

use app::prelude::*;

fn main() {
    let mut app: App<String> = App::new();
    app.add_system(&hello_world_sender)
        .add_system(&hello_world_receiver0)
        .add_system(&hello_world_receiver1)
        .run(String::from("!"));
}

fn hello_world_sender(a: String, s: AppSender<String>) {
    println!("World {}", a);
    loop {
        s.send("World".to_string()).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}
fn hello_world_receiver0(a: String, _: AppSender<String>) {
    println!("Hello {}", a)
}
fn hello_world_receiver1(a: String, _: AppSender<String>) {
    println!("Bye {}", a)
}
