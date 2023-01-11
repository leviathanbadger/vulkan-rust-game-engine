use std::collections::{HashSet};
use anyhow::{Result};

pub fn vec_from_hash_set<T>(hash: &HashSet<T>) -> Result<Vec<T>> where T : Copy + Clone {
    let mut vec = Vec::with_capacity(hash.len());

    for item in hash {
        vec.push(*item);
    }

    Ok(vec)
}
