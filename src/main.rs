mod args;
mod error;
mod hasher;
mod thread;

fn main() -> std::process::ExitCode {
    let start = std::time::Instant::now();
    if let Err(e) = fallible_main() {
        eprintln!("[31mERROR[m {e}");
        1
    } else {
        eprintln!("Elapsed: {:?}", start.elapsed());
        0
    }
    .into()
}

fn fallible_main() -> error::Result {
    let (left, right, verbose) = args::get()?;
    let pool = thread::pool(verbose)?;
    let left = hasher::Index::new(left, &pool)?;
    let right = hasher::Index::new(right, &pool)?;
    println!("{}: {}", left.path().display(), left.len());
    println!("{}: {}", right.path().display(), right.len());
    Ok(())
}
