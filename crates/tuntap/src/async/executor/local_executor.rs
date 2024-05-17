use std::{
    cell::RefCell, 
    collections::VecDeque, 
    future::Future, 
    marker::PhantomData, 
    mem, 
    pin::pin, 
    rc::Rc, 
    sync::Arc, 
    task::{
        Context, Poll, RawWaker, RawWakerVTable, Wake, Waker
    }, time::Duration 
};

use futures::future::{LocalBoxFuture, FutureExt};

use crate::r#async::reactor::Reactor;


thread_local! {
    pub static EX: RefCell<Executor> = RefCell::new(Executor::new());
}

/// block the main async task
pub fn block_on<F, O>(fut: F) -> O
where
    F: Future<Output = O> + 'static
{
    let mut fut = pin!(fut);
    let ewaker = empty_waker();
    let mut ecx = Context::from_waker(&ewaker);
    //
    loop {
        if let Poll::Ready(r) = fut.as_mut().poll(&mut ecx) {
            break r;
        }
        EX.with_borrow_mut(|ex| {
            while let Some(task) = ex.queue.pop() {
                let mut fut = task.fut.borrow_mut();
                let inner_waker = waker(task.clone());
                let mut inner_cx = Context::from_waker(&inner_waker);
                let _ = fut.as_mut().poll(&mut inner_cx);
            }
        });
        if let Poll::Ready(r) = fut.as_mut().poll(&mut ecx) {
            break r;
        }
        EX.with_borrow_mut(|ex| {
            ex.reactor.run_once();
        });
    }
}

pub fn swpan(fut: impl Future<Output = ()> + 'static) {
    EX.with_borrow_mut(|ex| {
        ex.queue.push(Rc::new(Task {
            fut: RefCell::new(fut.boxed_local())
        }));
    });
}

pub struct Executor {
    queue: TaskQueue,
    reactor: Reactor,
    // mark executor, mark it as !Sender and !Sync
    _mark: PhantomData<Rc<()>>,
}

impl Executor {
    fn new() -> Self {
        let queue = TaskQueue::new();
        let _mark = Default::default();
        let reactor =  Reactor::new(Duration::from_millis(2));
        Self {
            queue,
            reactor,
            _mark
        }
    }
}

fn waker(task: Rc<Task>) -> Waker {
    let raw_task = Rc::into_raw(task) as _;
    unsafe {
        Waker::from_raw(RawWaker::new(raw_task, &VTableTool::RAWVTABLE))
    }
}

struct Task {
    fut: RefCell<LocalBoxFuture<'static, ()>>
}

impl Task {

    #[inline(always)]
    fn wake(self: Rc<Self>) {
        Self::wake_by_ref(&self);
    }

    #[inline(always)]
    fn wake_by_ref(self: &Rc<Self>) {
        EX.with_borrow(|ex| {
            ex.queue.push(self.clone())
        });
    }
}

struct TaskQueue {
    inner: RefCell<VecDeque<Rc<Task>>>
}

impl TaskQueue {
    #[inline(always)]
    fn new() -> Self {
        let inner = RefCell::new(VecDeque::with_capacity(128));
        Self {
            inner
        }
    }
    
    #[inline(always)]
    fn push(&self, task: Rc<Task>) {
        self.inner.borrow_mut().push_back(task);
    }

    #[inline(always)]
    fn pop(&self) -> Option<Rc<Task>> {
        self.inner.borrow_mut().pop_front()
    }
}

fn empty_waker() -> Waker {
    Waker::from(Arc::new(EmptyWake))
}

struct EmptyWake;

impl Wake for EmptyWake {
    fn wake(self: std::sync::Arc<Self>) {
    }

    fn wake_by_ref(self: &std::sync::Arc<Self>) {
    }
}

struct VTableTool;

impl VTableTool {
    const RAWVTABLE: RawWakerVTable = RawWakerVTable::new( 
        Self::clone,
        Self::wake,
        Self::wake_by_ref,
        Self::drop,
    );

    /// clone the waker.
    unsafe fn clone(data: *const ()) -> RawWaker {
        Self::increase_ref(data);
        RawWaker::new(data, &Self::RAWVTABLE)
    }

    unsafe fn wake_by_ref(waker_ptr: *const ()) {
        let task = Rc::from_raw(waker_ptr as *const Task);
        task.wake_by_ref();
    }

    unsafe fn drop(waker_ptr: *const ()) {
        drop(Rc::from_raw(waker_ptr as *const Task))
    }

    unsafe fn wake(data: *const ()) {
        let task: Rc<Task> = Rc::from_raw(data as _);
        task.wake();
    }

    unsafe fn increase_ref(data: *const()) {
        let task = Rc::from_raw(data as *const Rc<Task>);
        let task = mem::ManuallyDrop::new(task);
        let _m = task.clone();
    }

}

#[cfg(test)]
mod test {

    use futures::executor::block_on;

    #[test]
    fn test_block_on() {
        let r = block_on(async {
            1
        });
        assert_eq!(r, 1);
    }
}