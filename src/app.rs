use crate::app_mpsc::*;
use std::thread;

pub mod prelude {
    pub use crate::app::*;
    pub use crate::app_mpsc::*;
}

pub struct App<T: 'static> {
    systems: Vec<&'static (dyn Fn(T, AppSender<T>) + Sync + Send)>,
    app_sender_factory: AppSenderFactory<T>,
    app_receiver: AppReceiver<T>,
}

impl<T> App<T>
where
    T: Send + Clone + 'static,
{
    pub fn new() -> Self {
        let (s, r) = AppMPSC::channel();
        App::<T> {
            systems: Vec::new(),
            app_sender_factory: s,
            app_receiver: r,
        }
    }
    pub fn add_system(&mut self, f: &'static (dyn Fn(T, AppSender<T>) + Sync + Send)) -> &mut Self {
        self.systems.push(f);
        self
    }
    pub fn run(&self, top: T) {
        self.app_sender_factory.build(usize::MAX).send(top).unwrap();
        for v in self.app_receiver.inner() {
            for (i, f) in self.systems.iter().enumerate() {
                let sender = self.app_sender_factory.build(i);
                let v = v.clone();
                let f = Clone::clone(f);
                if v.not(i) {
                    thread::spawn(move || f(v.inner(), sender));
                }
            }
        }
    }
}
