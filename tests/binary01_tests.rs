#[cfg(test)]
mod tests {

    #[test]
    fn result_present() {
        let list = [1, 3, 5, 7, 8];
        let target = 5;

        //should return 2
        assert_eq!(vanilla_binary(list, target), 2);
    }

    #[test]
    fn result_missing() {
        let list = [1, 2, 3, 4, 5, 6, 7];
        let target = 0;

        //should return -1
        assert_eq!(vanilla_binary(list, target), -1);
    }

    #[test]
    fn larger_array() {
        let list = [2, 8, 89, 120, 1000];
        let target = 120;

        //should return 3
        assert_eq!(vanilla_binary(list, target), 3);
    }

    #[test]
    fn smaller_array() {
        let list = [10, 20];
        let target = 20;

        //should return 1
        assert_eq!(vanilla_binary(list, target), 1);
    }

    #[test]
    fn smallest_array() {
        let mut list = [1];
        let target = 1;

        //should return 0
        assert_eq!(vanilla_binary(list, target), 0);
    }

    #[test]
    fn empty_array() {
        let list = [];
        let target = 1;

        //should return -1
        assert_eq!(vanilla_binary(list, target), -1);
    }

    #[test]
    fn not_found() {
        let list = [1, 2, 3, 4, 5];
        let target = 10;

        //should return -1
        assert_eq!(vanilla_binary(list, target), -1);
    }
}
