use super::entry;

pub fn first_pass(
    left: &mut Vec<entry::Entry>,
    right: &mut Vec<entry::Entry>,
    empty_hash: &entry::Hash,
    pool: &rayon::ThreadPool,
) {
    use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

    log::info!("Starting first pass");
    let start = std::time::Instant::now();

    let ptr = right.as_mut_ptr() as usize;
    pool.install(|| {
        left.par_iter_mut()
            .enumerate()
            .for_each(|(left_idx, entry)| {
                let ptr = ptr as *mut entry::Entry;
                match right.binary_search(entry) {
                    Ok(i) => {
                        entry.status = entry::Status::Same(i);
                        unsafe { (*ptr.add(i)).status = entry::Status::Same(left_idx) };
                    }
                    Err(_) if entry.hash() == empty_hash => {
                        entry.status = entry::Status::Empty;
                    }
                    Err(i) => {
                        let indices = matching_hashes(&entry.hash, i, right);
                        match indices.len() {
                            0 => {
                                if let Some(i) = right.iter().position(|e| e.path == entry.path) {
                                    entry.status = entry::Status::Modified(i);
                                    unsafe {
                                        (*ptr.add(i)).status = entry::Status::Modified(left_idx);
                                    }
                                } else {
                                    entry.status = entry::Status::Unique;
                                }
                            }
                            _ => entry.status = entry::Status::Maybe(indices),
                        }
                    }
                }
            });
    });

    log::info!("Finished first pass in {:?}", start.elapsed());
}

pub fn second_pass(
    left: &mut Vec<entry::Entry>,
    right: &mut Vec<entry::Entry>,
    empty_hash: &entry::Hash,
    pool: &rayon::ThreadPool,
) {
    use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

    log::info!("Starting second pass");
    let start = std::time::Instant::now();

    let ptr = left.as_mut_ptr() as usize;
    pool.install(|| {
        right
            .par_iter_mut()
            .filter(|e| matches!(e.status, entry::Status::Unique))
            .for_each(|entry| {
                let ptr = ptr as *mut entry::Entry;
                match left.binary_search(entry) {
                    Ok(i) => {
                        entry.status = entry::Status::Same(i);
                        log::warn!(
                            "Marking unexpected `SAME` on second pass for {}",
                            entry.path.display()
                        );
                    }
                    Err(_) if entry.hash() == empty_hash => {
                        entry.status = entry::Status::Empty;
                    }
                    Err(i) => {
                        let indices = matching_hashes(&entry.hash, i, left);
                        match indices.len() {
                            0 => entry.status = entry::Status::Unique,
                            1 => unsafe {
                                let left_idx = *indices.get_unchecked(0);
                                let correspondent = &mut *ptr.add(left_idx);
                                match &correspondent.status {
                                    entry::Status::Maybe(maybes) => {
                                        if maybes.len() == 1 {
                                            let right_idx = *maybes.get_unchecked(0);
                                            correspondent.status = entry::Status::Moved(right_idx);
                                        }
                                    }
                                    status => {
                                        log::warn!(
                                            "Expected `MAYBE` on left side during second pass for {}, but got{}",
                                            entry.path.display(),
                                            status,
                                        );
                                    }
                                }
                                entry.status = entry::Status::Moved(left_idx);
                            },
                            _ => entry.status = entry::Status::Maybe(indices),
                        }
                    }
                }
            });
    });

    log::info!("Finished second pass in {:?}", start.elapsed());
}

fn matching_hashes(hash: &entry::Hash, pivot: usize, children: &[entry::Entry]) -> Vec<usize> {
    let i = match children[..pivot].binary_search_by(|e| e.hash.cmp(&hash.decrement())) {
        Ok(i) | Err(i) => i,
    };

    children[i..]
        .iter()
        .take_while(|e| e.hash == *hash)
        .enumerate()
        .map(|(idx, _)| idx + i)
        .collect()
}
