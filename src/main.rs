use std::{any::Any, future::Future, pin::Pin, sync::mpsc::{self, Sender}, thread::{self, JoinHandle}};
use tokio::runtime;

fn main() {
    println!("Hello, world!");
    
    let rt = Runtime::new();
    rt.spawn(async { println!("Hello 1") });
    rt.spawn(async { println!("Hello 2") });
    rt.join();
}

type BoxFuture<T> = Pin<Box<dyn Future<Output=T> + Send>>;

struct Runtime {
    send: Sender<BoxFuture<()>>,
    handle: JoinHandle<()>
}

impl Runtime {
    fn new() -> Self {
        let (send, recv) = mpsc::channel::<BoxFuture<()>>();
        
        let handle = thread::spawn(move || {
            for msg in recv.iter() {
                async_std::task::spawn(msg);
            }
        });

        Self { send, handle }
    }

    fn spawn<F: 'static + Send + Future<Output = ()>>(&self, f: F) {
        let _ = self.send.send(
            Box::pin(f)
        );
    }

    fn join(self) -> Result<(), Box<(dyn Any + std::marker::Send + 'static)>> {
        self.handle.join()
    }
}
