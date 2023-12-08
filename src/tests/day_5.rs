
#[cfg(test)]
mod tests {
    use crate::days::day_6::*;

    #[test]
    fn test_process_line() {
        let line = "1 2 3 4 5";
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(process_line(line.to_string()), expected);

        let line = "";
        let expected = vec![];
        assert_eq!(process_line(line.to_string()), expected);
    }

    #[test]
    fn test_get_nums_wins() {
        let record = 10;
        let time = 5;
        let expected = 2;
        assert_eq!(get_nums_wins(record, time), expected);

        let record = 15;
        let time = 6;
        let expected = 3;
        assert_eq!(get_nums_wins(record, time), expected);
    }

    #[test]
    fn test_will_win() {
        let val = 2;
        let record = 4;
        let time = 2;
        assert!(will_win(val, record, time));

        let val = 1;
        let record = 3;
        let time = 2;
        assert!(!will_win(val, record, time));
    }

    #[test]
    fn test_find_win() {
        let record = 10;
        let time = 5;
        let expected = 3;
        assert_eq!(find_win(record, time), expected);

        let record = 15;
        let time = 6;
        let expected = 4;
        assert_eq!(find_win(record, time), expected);
    }
}