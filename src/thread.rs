#[derive(Debug, thiserror::Error)]
#[error("Failed to create thread pool: {0}")]
pub struct Error(#[from] rayon::ThreadPoolBuildError);

pub fn pool(verbose: bool) -> Result<rayon::ThreadPool, Error> {
    let pool = rayon::ThreadPoolBuilder::new().build()?;
    if verbose {
        eprintln!(
            "Creating thread pool with {} threads",
            pool.current_num_threads()
        );
    }
    Ok(pool)
}
