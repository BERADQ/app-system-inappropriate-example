use std::sync::mpsc::{channel, Receiver, RecvError, SendError, Sender};
use std::sync::Arc;
use std::sync::RwLock;

pub struct ArcWithID<T>(Arc<RwLock<T>>, usize);
impl<T> Clone for ArcWithID<T> {
    fn clone(&self) -> Self {
        ArcWithID(self.0.clone(), self.1.clone())
    }
}
impl<T> ArcWithID<T> {
    pub fn not(&self, id: usize) -> bool {
        self.1 != id
    }
    pub fn inner(&self) -> Arc<RwLock<T>> {
        self.0.clone()
    }
}
pub struct AppSender<T>(Sender<ArcWithID<T>>, usize);
impl<T> AppSender<T> {
    pub fn send(&self, something: T) -> Result<(), SendError<ArcWithID<T>>> {
        self.0
            .send(ArcWithID(Arc::new(RwLock::new(something)), self.1))
    }
}

pub struct AppSenderFactory<T>(Sender<ArcWithID<T>>);
impl<T> AppSenderFactory<T> {
    pub fn build(&self, id: usize) -> AppSender<T> {
        AppSender(self.0.clone(), id)
    }
}

pub struct AppReceiver<T>(Receiver<ArcWithID<T>>);
impl<T> AppReceiver<T> {
    #[allow(dead_code)]
    pub fn recv(&self) -> Result<ArcWithID<T>, RecvError> {
        self.0.recv()
    }
    pub fn inner(&self) -> &Receiver<ArcWithID<T>> {
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
