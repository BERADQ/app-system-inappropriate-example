mod app;
mod app_mpsc;

use std::thread;
use std::time::Duration;

use app::prelude::*;

#[derive(Clone, PartialEq, Eq)]
enum Test {
    Hello(String),
    Bye(String),
    Start,
    Stop,
}

fn main() {
    let mut app: App<Test> = App::new();
    app.set(Setting {
        stop_symbol: Some(Test::Stop),
    })
    .add_system(&hello_world_sender)
    .add_system(&hello_world_receiver1)
    .add_system_mut(&hello_world_receiver0)
    .run(Test::Start);
}

fn hello_world_sender(a: &Test, s: AppSender<Test>) {
    match a {
        Test::Start => {
            println!("Start!");
            let mut i = 0;
            loop {
                s.send(Test::Hello(format!("World {}", i))).unwrap();
                s.send(Test::Bye(format!("Dev {}", i))).unwrap();
                if i == 5 {
                    s.send(Test::Stop).unwrap();
                    break;
                }
                i += 1;
                thread::sleep(Duration::from_secs(1));
            }
        }
        _ => {}
    }
}
fn hello_world_receiver0(a: &mut Test, _: AppSender<Test>) {
    match a {
        Test::Hello(s) => {
            println!("Hello {}", s);
        }
        _ => {}
    }
}
fn hello_world_receiver1(a: &Test, _: AppSender<Test>) {
    match a {
        Test::Bye(s) => {
            println!("Bye {}", s);
        }
        _ => {}
    }
}
