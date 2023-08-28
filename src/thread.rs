// #[derive(Debug, thiserror::Error)]
// #[error("Failed to create thread pool: {0}")]
// pub struct Error(#[from] threadpool::ThreadPoolBuildError);

#[derive(Debug, thiserror::Error)]
#[error("Panic in spawned thread ({0}): {1:?}")]
pub struct ThreadPanic(pub String, pub Box<dyn std::any::Any + Send + 'static>);

// TODO: Use rayon
pub fn pool(verbose: bool) -> threadpool::ThreadPool {
    let pool = threadpool::Builder::new().build();
    if verbose {
        eprintln!("Creating thread pool with {} threads", pool.max_count());
    }
    pool
}
