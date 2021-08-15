use std::iter::repeat;

use itertools::Itertools;

pub fn q(n: usize) -> String {
    repeat("?").take(n).join(", ")
}
