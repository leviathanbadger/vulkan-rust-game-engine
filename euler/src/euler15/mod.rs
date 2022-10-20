mod tests;

fn lattice_path_count(size: usize) -> u64 {
    let mut data = vec![vec![0u64; size + 2]; size + 2];
    data[size + 1][size] = 1;

    for q in 0..(size + 1) {
        for w in 0..(size + 1) {
            data[size - q][size - w] = data[size + 1 - q][size - w] + data[size - q][size + 1 - w];
        }
    }

    data[0][0]
}

#[allow(dead_code)]
pub fn euler15() -> u64 {
    let result = lattice_path_count(20);

    result
}
