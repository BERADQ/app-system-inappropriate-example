use crate::app_mpsc::*;
use std::thread::{self, JoinHandle};

pub mod prelude {
    pub use crate::app::*;
    pub use crate::app_mpsc::*;
}

pub enum System<T: 'static> {
    S(&'static (dyn Fn(&T, AppSender<T>) + Sync + Send)),
    SMut(&'static (dyn Fn(&mut T, AppSender<T>) + Sync + Send)),
}

impl<T> Clone for System<T> {
    fn clone(&self) -> Self {
        match self {
            Self::S(f) => Self::S(Clone::clone(f)),
            Self::SMut(f) => Self::SMut(Clone::clone(f)),
        }
    }
}

pub struct App<T: 'static + Send + Sync + Eq> {
    systems: Vec<System<T>>,
    app_sender_factory: AppSenderFactory<T>,
    app_receiver: AppReceiver<T>,
    settings: Setting<T>,
}
pub struct Setting<T> {
    pub stop_symbol: Option<T>,
}
impl<T> Default for Setting<T> {
    fn default() -> Self {
        Setting { stop_symbol: None }
    }
}

/*impl<T, F> From<&'static F> for System<T>
where
    F: Fn(&T, AppSender<T>) + Sync + Send,
{
    fn from(value: &'static F) -> Self {
        Self::S(value)
    }
}*/
/*impl<T, F> From<&'static F> for System<T>
where
    F: Fn(&mut T, AppSender<T>) + Sync + Send,
{
    fn from(value: &'static F) -> Self {
        Self::SMut(value)
    }
}*/

impl<T> App<T>
where
    T: Send + Sync + Eq,
{
    pub fn new() -> Self {
        let (s, r) = AppMPSC::channel();
        App::<T> {
            systems: Vec::new(),
            app_sender_factory: s,
            app_receiver: r,
            settings: Setting::<T>::default(),
        }
    }
    pub fn set(&mut self, s: Setting<T>) -> &mut Self {
        self.settings = s;
        self
    }
    pub fn add_system<F>(&mut self, f: &'static F) -> &mut Self
    where
        F: Fn(&T, AppSender<T>) + Sync + Send,
    {
        self.systems.push(System::S(f));
        self
    }
    pub fn add_system_mut<F>(&mut self, f: &'static F) -> &mut Self
    where
        F: Fn(&mut T, AppSender<T>) + Sync + Send,
    {
        self.systems.push(System::SMut(f));
        self
    }
    pub fn run(&self, top: T) {
        self.app_sender_factory.build(usize::MAX).send(top).unwrap();
        let mut joinhandles: Vec<Option<JoinHandle<()>>> = Vec::new();
        'a: for v in self.app_receiver.inner() {
            match &self.settings.stop_symbol {
                Some(s) => {
                    if v.inner().read().unwrap().eq(s) {
                        for jh in &mut joinhandles {
                            match jh {
                                Some(_) => jh.take().unwrap().join().unwrap(),
                                None => {}
                            }
                            break 'a;
                        }
                    }
                }
                None => {}
            }
            for (i, f) in self.systems.iter().enumerate() {
                let sender = self.app_sender_factory.build(i);
                let v = v.clone();
                let f = Clone::clone(f);
                if v.not(i) {
                    match f {
                        System::SMut(f) => {
                            joinhandles.push(Some(thread::spawn(move || {
                                f(&mut v.inner().write().unwrap(), sender)
                            })));
                        }
                        System::S(f) => {
                            joinhandles.push(Some(thread::spawn(move || {
                                f(&v.inner().read().unwrap(), sender)
                            })));
                        }
                    }
                }
            }
        }
    }
}
