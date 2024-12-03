use std::iter::zip;

fn main() {
    let file = include_str!("../part1Input.txt");
    let answer = solution(file);
    println!("{}", answer);
}

fn solution(input: &str) -> i32 {
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        let num1 = numbers.next().unwrap();
        let num2 = numbers.next().unwrap();

        list1.push(num1.parse().unwrap());
        list2.push(num2.parse().unwrap());
    }

    list1.sort();
    list2.sort();
    let mut total_distance: i32 = 0;

    for pair in zip(list1, list2) {
        total_distance += (pair.0 - pair.1).abs();
    }

    return total_distance;
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn basic_test() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let result = solution(input);

        assert_eq!(result, 11);
    }
}
