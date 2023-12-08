use std::sync::mpsc::{channel, Receiver, RecvError, SendError, Sender};

#[derive(Clone)]
pub struct WithID<T>(T, usize);
impl<T: Clone> WithID<T> {
    pub fn not(&self, id: usize) -> bool {
        self.1 != id
    }
    pub fn inner(&self) -> T {
        self.0.clone()
    }
}
pub struct AppSender<T>(Sender<WithID<T>>, usize);
impl<T> AppSender<T> {
    pub fn send(&self, something: T) -> Result<(), SendError<WithID<T>>> {
        self.0.send(WithID(something, self.1))
    }
}

pub struct AppSenderFactory<T>(Sender<WithID<T>>);
impl<T> AppSenderFactory<T> {
    pub fn build(&self, id: usize) -> AppSender<T> {
        AppSender(self.0.clone(), id)
    }
}

pub struct AppReceiver<T>(Receiver<WithID<T>>);
impl<T> AppReceiver<T> {
    #[allow(dead_code)]
    pub fn recv(&self) -> Result<WithID<T>, RecvError> {
        self.0.recv()
    }
    pub fn inner(&self) -> &Receiver<WithID<T>> {
        &self.0
    }
}

pub struct AppMPSC;
impl AppMPSC {
    pub fn channel<T>() -> (AppSenderFactory<T>, AppReceiver<T>) {
        let (s, r) = channel();
        (AppSenderFactory(s), AppReceiver(r))
    }
}
