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
    let pool = thread::pool(verbose);
    // let left_index = hasher::DirIndex::new(&left, &pool)?;
    // let right_index = hasher::DirIndex::new(&right, &pool)?;
    // println!("{}", left_index.len());
    // println!("{}", right_index.len());
    Ok(())
}
