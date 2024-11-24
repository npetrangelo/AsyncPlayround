use std::{future::Future, pin::Pin, process::Output, sync::{mpsc::{self, Sender}, Arc}, thread::{self, JoinHandle}};
use tokio::runtime;

fn main() {
    println!("Hello, world!");
    
    let rt = Runtime::new();
    rt.spawn(async { println!("Hello 1") });
    rt.spawn(async { println!("Hello 2") });
    rt.join();
}

type BoxFuture<T> = Pin<Box<dyn Future<Output=T> + Send>>;
type FuncType<T> = Box<(dyn Fn(&mut T) -> BoxFuture<()> + Send)>;

struct Runtime {
    send: Sender<FuncType<()>>,
    handle: JoinHandle<()>
}

impl Runtime {
    fn new() -> Self {
        let (send, recv) = mpsc::channel::<FuncType<()>>();
        
        let handle = thread::spawn(move || {
            for msg in recv.iter() {
                let foo = msg(&mut ());
                async_std::task::spawn(foo);
            }
        });

        Self { send, handle }
    }

    fn spawn<F: 'static + Send + Future<Output = ()>>(&self, f: F) {
        let _ = self.send.send(
            Box::new(
                move |_| Box::pin(f)
            )
        );
    }

    fn join(self) {
        self.handle.join();
    }
}
