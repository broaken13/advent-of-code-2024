fn main() {
    println!("Part 2!");
    let input = include_str!("../day2input.txt");
    let answer = solution(input);
    println!("There are {answer} safe reports");
}

fn solution(input: &str) -> i32 {
    let mut count = 0;

    for line in input.lines() {
        if test_report(line) {
            count += 1;
        }
    }

    return count;
}

fn test_report(line: &str) -> bool {
    let numbers = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    if test_sequence(&numbers) {
        return true;
    }

    for i in 0..numbers.len() {
        let mut skipped_list = numbers.clone();
        skipped_list.remove(i);
        if test_sequence(&skipped_list) {
            return true;
        }
    }

    return false;
}

fn test_sequence(numbers: &Vec<i32>) -> bool {
    let mut iterator = numbers.iter();

    let mut previous = iterator.next().unwrap();
    let mut num = iterator.next().unwrap();

    let is_increasing = num > previous;

    if is_increasing {
        if *num <= *previous || *num > *previous + 3 {
            return false;
        }
    } else {
        if *num >= *previous || *num < *previous - 3 {
            return false;
        }
    }

    for next_num in iterator {
        previous = num;
        num = &next_num;

        println!("Comparing {previous} to {num}");
        if is_increasing {
            if *num <= *previous || *num > *previous + 3 {
                return false;
            }
        } else {
            if *num >= *previous || *num < *previous - 3 {
                return false;
            }
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use crate::{solution, test_report, test_sequence};

    #[test]
    fn basic_test() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let answer = solution(input);
        assert_eq!(answer, 4);
    }

    #[test]
    fn sequencer() {
        let seq = vec![1,2,3];
        assert!(test_sequence(&seq), "Seq: {seq:?}");
    }

    #[test]
    fn line_test_normal() {
        let line = "7 6 4 2 1";
        assert!(test_report(line));
    }

    #[test]
    fn line_test_skip() {
        let mut line = "1 2 2 3 5";
        assert!(test_report(line), "Line: {line}");

        line = "1 3 2 4 5";
        assert!(test_report(line), "Line: {line}");

        line = "14 14 17 19 22";
        assert!(test_report(line), "Line: {line}");
    }

    #[test]
    fn line_test_bad() {
        let line = "1 2 7 8 9";
        assert!(!test_report(line));
    }
    
    #[test]
    fn line_test_end() {
        let line = "21 18 15 12 11 9 8 1";
        assert!(test_report(line));
    }

    #[test]
    fn line_test_beginning() {
        let line = "1 5 6 7 8";
        assert!(test_report(line), "Line: {line}");
    }
}

