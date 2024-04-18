use std::cmp::Ordering::*;

pub fn vanilla_binary(list: &[usize], target: usize) -> Option<usize> {
    //returns option Some with wrapped index of target
    //or option None if not found
    let mut left: usize = 0;
    if list.is_empty() {
        //accounting for the empty array
        return None;
    }
    let mut right: usize = list.len() - 1;

    let mut mid: usize = (right + left) / 2;

    while left <= right {
        match list[mid].cmp(&target) {
            Equal => return Some(mid),
            Less => left = mid + 1,
            Greater => {
                if right < 1 {
                    break;
                } else {
                    right = mid - 1
                }
            }
        }
        mid = (left + right) / 2;
    }

    None
}
