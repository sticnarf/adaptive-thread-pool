use std::future::Future;

pub trait AdaptiveSpawn: Clone {
    fn spawn_opt<Fut>(&self, f: Fut, options: Options)
    where
        Fut: Future<Output = ()> + Send + 'static;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Options {
    /// A unique identifier of a meta-task.
    ///
    /// Independent `Future`s of a single meta-task share the same token.
    pub token: u64,

    /// Like the nice in UNIX, bigger value means lower priority, but the range is [0, 255].
    pub nice: u8,
}
