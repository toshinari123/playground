pub fn merge_sort(xs: &[i32]) -> Vec<i32> {
    match xs[..] {
        [] => vec![],
        [x] => vec![x],
        [x, y] => {
            if x < y {
                vec![x, y]
            } else {
                vec![y, x]
            }
        }
        ref xs => {
            let (left, right) = split(xs);
            merge(&merge_sort(left), &merge_sort(right))
        }
    }
}

fn split(xs: &[i32]) -> (&[i32], &[i32]) {
    (&xs[0..xs.len() / 2], &xs[xs.len() / 2..])
}

fn merge(xs: &[i32], ys: &[i32]) -> Vec<i32> {
    let target_len = xs.len() + ys.len();
    let mut merged = vec![];
    let mut x_index = 0;
    let mut y_index = 0;
    while merged.len() < target_len {
        let (x, y) = (xs.get(x_index), ys.get(y_index));
        match (x, y) {
            (Some(x), Some(y)) => {
                if x < y {
                    merged.push(*x);
                    x_index += 1;
                } else {
                    merged.push(*y);
                    y_index += 1;
                }
            }
            (Some(x), None) => {
                merged.push(*x);
                x_index += 1;
            }
            (None, Some(y)) => {
                merged.push(*y);
                y_index += 1
            }
            (None, None) => {}
        }
    }
    merged
}

#[cfg(test)]
pub mod test {
    use stdext::prelude::Assertable;

    use crate::comp3257::merge_sort::merge_sort;

    #[test]
    pub fn test1() {
        merge_sort(&[5, 0, 3, 1, 2, 4]).must_be(vec![0, 1, 2, 3, 4, 5]);
        merge_sort(&[]).must_be(Vec::<i32>::new());
        merge_sort(&[12, 5, 67, 32, 20, 9, 39, 78]).must_be(vec![5, 9, 12, 20, 32, 39, 67, 78]);
    }
}
