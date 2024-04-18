mod b02;
mod b02_tests;

use crate::b02::find_boundary;

fn main() {
    let list: [bool; 5] = [false, false, true, true, true];

    //should return 2
    assert_eq!(find_boundary(&list).unwrap_or_else(1.5), 2);
}
