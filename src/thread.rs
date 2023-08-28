#[derive(Debug, thiserror::Error)]
#[error("Failed to create thread pool: {0}")]
pub struct Error(#[from] rayon::ThreadPoolBuildError);

pub fn pool() -> Result<rayon::ThreadPool, Error> {
    let pool = rayon::ThreadPoolBuilder::new().build()?;
    log::info!(
        "Creating thread pool with {} threads",
        pool.current_num_threads()
    );
    Ok(pool)
}
