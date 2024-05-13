mod reactor;
mod executor;
mod async_dev;

#[cfg(test)]
mod test {
    use std::{
        sync::{
            atomic::AtomicI32,
            Arc, 
            Mutex, 
            OnceLock,
        }, 
        task::{
            Poll, Waker
        }, 
        thread, time::Duration, 
    };

    use futures::{executor::{self, ThreadPoolBuilder}, task::SpawnExt, Future};

    fn get_queue() -> Arc<Mutex<Vec<Waker>>> {
        static L: OnceLock<Arc<Mutex<Vec<Waker>>>> = OnceLock::new();
        L.get_or_init(|| {
            Arc::new(Mutex::new(Vec::new()))
        }).clone()
    }


    struct A(i32);

    impl Future for A {
        type Output = ();
    
        fn poll(
            self: std::pin::Pin<&mut Self>, 
            cx: &mut std::task::Context<'_>
        ) -> std::task::Poll<Self::Output> 
        {
            let t1 = get_t1_count();
            get_queue().lock().unwrap().push(cx.waker().clone());
            println!("{}  t1 {t1} {:?}", self.0, thread::current().id());
            if  t1 > 10 {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
            
        }
    }

    fn get_t1_count() -> i32 {
        static COUNT: AtomicI32 = AtomicI32::new(0);
        COUNT.fetch_add(1, std::sync::atomic::Ordering::Acquire)
    }

    fn get_t2_count() -> i32 {
        static COUNT: AtomicI32 = AtomicI32::new(0);
        COUNT.fetch_add(1, std::sync::atomic::Ordering::Acquire)
    }

    #[test]
    fn test_future() {
        thread::spawn(|| {
            loop {
                if let Some(w) = get_queue().lock().unwrap().pop() {
                    let t2 = get_t2_count();
                    println!("wake task {t2} {:?}", thread::current().id());
                    if t2 == 50 {
                        break;
                    }
                    w.wake();
                } else {
                    thread::sleep(Duration::from_millis(1));
                    continue;
                }
            }
        });
        let pool = ThreadPoolBuilder::new()
            .pool_size(2)
            .create()
            .unwrap();
        let a1 = A(1);
        let a2 = A(2);
        pool.spawn(async {
            a1.await;
        }).unwrap();
        executor::block_on(async {
            a2.await;
        });
        assert!(get_t1_count()>=get_t2_count());
    }
}