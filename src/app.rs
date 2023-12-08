use crate::app_mpsc::*;
use std::thread;

pub mod prelude {
    pub use crate::app::*;
    pub use crate::app_mpsc::*;
}

pub struct App<T> {
    systems: Vec<*mut (dyn Fn(T, AppSender<T>) + Sync + Send + 'static)>,
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
    pub fn add_system<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(T, AppSender<T>) + Sync + Send + 'static,
    {
        self.systems.push(Box::into_raw(Box::new(f)));
        self
    }
    pub fn run(&self, top: T) {
        self.app_sender_factory.build(usize::MAX).send(top).unwrap();
        for v in self.app_receiver.inner() {
            for (i, f) in self.systems.iter().enumerate() {
                let sender = self.app_sender_factory.build(i);
                let f = unsafe { Box::from_raw(f.clone()) };
                let v = v.clone();
                if v.not(i) {
                    thread::spawn(move || f(v.inner(), sender));
                }
            }
        }
    }
}
