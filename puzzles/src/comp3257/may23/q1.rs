use std::fmt::Display;

use stdext::prelude::*;

// Various organizations rank universities worldwide every year. These rankings
// are often controversial and do not always agree with each other. Alice wants to find out
// the level of (in)consistency between the QS ranking and the US News ranking. Consider n
// universities. For each university 1 < i < n, qi and ui; are its ranks accoording to QS and US
// News respectively. (Hence, q1, q2, ..., qn and u1, u2,...un are two permutations of 1 to n.)
// We call a pair of universities i, j a “disagreement” if qi > qj; and ui < uj. Design a divide
// and conquer algorithm for finding the number of “disagreements” between the QS and US
// News rankings. Your algorithm shall have running time strictly faster than O(n^2); to get
// full credit, your algorithm shall have running time O(nlogn).

fn find_in_u(i: u32, u: &Vec<u32>) -> usize {
    for (index, x) in u.iter().enumerate() {
        if &i == x {
            return index;
        }
    }
    unreachable!()
}

fn algo(q: Vec<u32>, u: Vec<u32>) -> u32 {
    let mut no_disagreements = 0;
    // for (index_in_q, x) in q.iter().enumerate() {
    //     let index_in_u = find_in_u(x, &u);
    //     if index_in_q
    // }
    0
}

pub fn answer() -> impl Display {
    algo(vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1])
}
