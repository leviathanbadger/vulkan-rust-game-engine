mod tests;

#[macro_export]
macro_rules! tree_path {
    ( $( $( $val:expr )* );* ) => {
        [
            $(
                vec![
                    $(
                        $val
                    ),*
                ]
            ),*
        ]
    }
}

fn euler18_tree() -> [Vec<i32>; 15] {
    tree_path!(
        75;
        95 64;
        17 47 82;
        18 35 87 10;
        20 04 82 47 65;
        19 01 23 75 03 34;
        88 02 77 73 07 63 67;
        99 65 04 28 06 16 70 92;
        41 41 26 56 83 40 80 70 33;
        41 48 72 33 47 32 37 16 94 29;
        53 71 44 65 25 43 91 52 97 51 14;
        70 11 33 28 77 73 17 78 39 68 17 57;
        91 71 52 38 17 14 91 43 58 50 27 29 48;
        63 66 04 68 89 53 67 30 73 16 69 87 40 31;
        04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
    )
}

pub fn minimum_path_sum<const SIZE: usize>(mut tree: [Vec<i32>; SIZE]) -> i32 {
    for q in (0..(SIZE - 1)).rev() {
        let len = tree[q].len();
        for w in 0..len {
            let max: i32 = {
                let next = &tree[q + 1];
                i32::max(next[w], next[w + 1])
            };
            let vec = &mut tree[q];
            vec[w] += max;
        }
    }

    tree[0][0]
}

#[allow(dead_code)]
pub fn euler18() -> i32 {
    let result = minimum_path_sum(euler18_tree());

    result
}
