pub mod binary {

    pub fn vanilla_binary(list: &[usize], target: usize) -> Option<usize> {
        //returns index of target
        //or option None if not found
        let mut left: usize = 0;
        let mut right: usize = list.len() - 1;

        let mut mid: usize = (right - left) / 2;

        while left <= right {
            if list[mid] == target {
                return Some(mid);
            }
        }

        None
    }
}
