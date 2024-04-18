mod b02;
mod b02_tests;

use crate::b02::find_boundary;

fn main() {
    let list: [bool; 5] = [true, true, true, true, true];

    //should return 0
    assert_eq!(find_boundary(&list), Some(0));
}
