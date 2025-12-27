use core::pin::{ pin, Pin };
use core::future::{ Future };
use core::task::{ self, Context, Waker };
use core::ops::{ Deref, DerefMut };

pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub trait Task {
    type Output;
    fn poll(&mut self) -> Poll<Self::Output>;
}

pub struct TaskToFuture<T: Task> {
    task: T,
}

impl<T: Task> TaskToFuture<T> {
    pub fn new(task: T) -> Self {
        Self {
            task: task,
        }
    }
}

impl<T: Task> Deref for &mut TaskToFuture<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.task
    }
}

impl<T: Task> Future for TaskToFuture<T> {
    type Output = T::Output;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> task::Poll<Self::Output> {
        match self.task.poll() {
            Poll::Ready(ret) => task::Poll::Ready(ret),
            Poll::Pending => task::Poll::Pending,
        }
    }
}

/*
pub struct RawRuntime<'a, T: Future> {
    topin: T,
    ctx: Context<'a>,
}

impl<'a, T: Future> RawRuntime<'_, T> {
    pub fn new(task: T) -> Self {
        Self {
            topin: task,
            ctx: Context::from_waker(Waker::noop()),
        }
    }

    pub fn run(&mut self) {
        let task = pin!(&mut self.topin);
        task.poll(&mut self.ctx);
    }
}
*/
