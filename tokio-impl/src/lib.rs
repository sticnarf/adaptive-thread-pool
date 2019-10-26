use adaptive_spawn::*;
use futures::future::{Future, FutureExt, TryFutureExt};
use std::sync::Arc;

#[derive(Clone)]
pub struct ThreadPool {
    inner: Arc<tokio_threadpool::ThreadPool>,
}

impl ThreadPool {
    pub fn new(inner: tokio_threadpool::ThreadPool) -> Self {
        ThreadPool {
            inner: Arc::new(inner),
        }
    }
}

impl AdaptiveSpawn for ThreadPool {
    fn spawn_opt<Fut>(&self, f: Fut, _: Options)
    where
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.inner.spawn(f.unit_error().boxed().compat());
    }
}
