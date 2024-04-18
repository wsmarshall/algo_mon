mod binary01;
mod binary01_tests;

use crate::binary01::vanilla_binary;

fn main() {
    let list: [usize; 7] = [1, 2, 3, 4, 5, 6, 7];
    let target: usize = 0;

    //should return None
    assert_eq!(vanilla_binary(&list, target), None);
}
