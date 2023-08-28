mod entry;
mod error;
mod index;
mod thread;

pub use entry::{Entry, Status};
pub use error::Error;
pub use index::Index;

/// Compares two directories [`left`](std::path::PathBuf) and [`right`](std::path::PathBuf)
/// returning the [`Index`](index::Index)
///
/// # Errors
///
/// This is a fallible process and will fail-fast.
/// In rare occasions, i.e. when worker threads are not able to send errors back up orchestrator,
/// errors will be globbed and simply written into an [`Error`](log::Level::Error) log entry.
pub fn compare(
    left: std::path::PathBuf,
    right: std::path::PathBuf,
) -> Result<(index::Index, index::Index), error::Error> {
    let pool = thread::pool()?;
    let mut left = index::Index::new(left, &pool)?;
    let mut right = index::Index::new(right, &pool)?;

    first_pass(&mut left, &mut right, &pool);
    second_pass(&mut left, &mut right, &pool);
    // clean_pair(&left, &mut right, &pool);
    // for (left_idx, e) in left.children.iter().enumerate() {
    //     match e.status {
    //         entry::Status::Same(right_idx) => unsafe {
    //             right.children.get_unchecked_mut(right_idx).status = entry::Status::Same(left_idx);
    //         },
    //         entry::Status::Modified(right_idx) => unsafe {
    //             right.children.get_unchecked_mut(right_idx).status = entry::Status::Modified(left_idx);
    //         },
    //         _ => {}
    //     }
    // }

    Ok((left, right))
}

fn first_pass(left: &mut index::Index, right: &mut index::Index, pool: &rayon::ThreadPool) {
    use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

    let ptr = right.children.as_mut_ptr() as usize;
    pool.install(|| {
        left.children
            .par_iter_mut()
            .enumerate()
            .for_each(|(left_idx, entry)| {
                let ptr = ptr as *mut entry::Entry;
                match right.children.binary_search(entry) {
                    Ok(i) => {
                        entry.status = entry::Status::Same(i);
                        unsafe { (*ptr.add(i)).status = entry::Status::Same(left_idx) };
                    }
                    Err(i) => {
                        // let i = match right.children[..i]
                        //     .binary_search_by(|e| e.hash.cmp(&entry.hash.decrement()))
                        // {
                        //     Ok(i) | Err(i) => i,
                        // };
                        //
                        // let indices = right.children[i..]
                        //     .iter()
                        //     .take_while(|e| e.hash == entry.hash)
                        //     .enumerate()
                        //     .map(|(idx, _)| idx + i)
                        //     .collect::<Vec<_>>();
                        let indices = matching_hashes(&entry.hash, i, right.children());
                        match indices.len() {
                            0 => {
                                if let Some(i) =
                                    right.children().iter().position(|e| e.path == entry.path)
                                {
                                    entry.status = entry::Status::Modified(i);
                                    unsafe {
                                        (*ptr.add(i)).status = entry::Status::Modified(left_idx);
                                    }
                                } else {
                                    entry.status = entry::Status::Unique;
                                }
                            }
                            // 1 => {
                            //     entry.status =
                            //         entry::Status::Moved(unsafe { *indices.get_unchecked(0) });
                            // }
                            _ => entry.status = entry::Status::Maybe(indices),
                        }
                    }
                }
            });
    });
}

fn second_pass(left: &mut index::Index, right: &mut index::Index, pool: &rayon::ThreadPool) {
    use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

    let ptr = left.children.as_mut_ptr() as usize;
    pool.install(|| {
        right
            .children
            .par_iter_mut()
            .filter(|e| matches!(e.status, entry::Status::Unique))
            .for_each(|entry| {
                let ptr = ptr as *mut entry::Entry;
                match left.children.binary_search(entry) {
                    Ok(i) => {
                        entry.status = entry::Status::Same(i);
                        log::warn!("Found `SAME` on second pass for {}", entry.path.display());
                    }
                    Err(i) => {
                        let indices = matching_hashes(&entry.hash, i, left.children());
                        match indices.len() {
                            0 => entry.status = entry::Status::Unique,
                            1 => unsafe {
                                let left_idx = *indices.get_unchecked(0);
                                entry.status = entry::Status::Moved(left_idx);
                                let correspondent = &mut *ptr.add(left_idx);
                                if let entry::Status::Maybe(maybes) = &correspondent.status {
                                    if maybes.len() == 1 {
                                        let right_idx = *maybes.get_unchecked(0);
                                        correspondent.status = entry::Status::Moved(right_idx);
                                    }
                                }
                            },
                            _ => entry.status = entry::Status::Maybe(indices),
                        }
                    }
                }
            });
    });
}

fn matching_hashes(hash: &entry::Hash, pivot: usize, children: &[entry::Entry]) -> Vec<usize> {
    let i = match children[..pivot].binary_search_by(|e| e.hash.cmp(hash)) {
        Ok(i) | Err(i) => i,
    };

    children[i..]
        .iter()
        .take_while(|e| e.hash == *hash)
        .enumerate()
        .map(|(idx, _)| idx + i)
        .collect()
}

// fn clean_pair(left: &index::Index, right: &mut index::Index, pool: &rayon::ThreadPool) {
//     use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
//
//     let ptr = right.children.as_mut_ptr() as usize;
//     pool.install(|| {
//         // for (left_idx, e) in
//         left.children
//             .par_iter()
//             .enumerate()
//             .for_each(|(left_idx, e)| {
//                 let ptr = ptr as *mut entry::Entry;
//                 match e.status {
//                     entry::Status::Same(right_idx) => unsafe {
//                         (*ptr.add(right_idx)).status = entry::Status::Same(left_idx);
//                         // right.children.get_unchecked_mut(right_idx).status = entry::Status::Same(left_idx);
//                     },
//                     entry::Status::Modified(right_idx) => unsafe {
//                         (*ptr.add(right_idx)).status = entry::Status::Modified(left_idx);
//                         // right.children.get_unchecked_mut(right_idx).status = entry::Status::Modified(left_idx);
//                     },
//                     _ => {}
//                 }
//             });
//     });
// }
