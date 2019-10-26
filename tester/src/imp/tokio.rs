use tokio_impl::ThreadPool;

pub fn init() -> ThreadPool {
    let inner = tokio_threadpool::ThreadPool::new();
    ThreadPool::new(inner)
}
