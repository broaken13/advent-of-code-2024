use std::collections::HashMap;

fn main() {
    println!("part 2!");
    let input = include_str!("../part1Input.txt");
    let answer = solution(input);
    println!("Similarity Score is {}", answer);
}

fn solution(input: &str) -> i32 {
    let mut left_list = vec![];
    let mut number_count = HashMap::new();

    for line in input.lines() {
        let mut split = line.split("   ");

        left_list.push(split.next().unwrap().parse::<i32>().unwrap());
        let right_number = split.next().unwrap().parse::<i32>().unwrap();

        let count = number_count.entry(right_number).or_insert(0);
        *count += 1;
    }

    let mut similarity_score = 0;

    for num in left_list {
        if number_count.contains_key(&num) {
            similarity_score += number_count.get(&num).unwrap() * num;
        }
    }
     
    return similarity_score;
}


#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn part2_test() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let answer = solution(input);
        assert_eq!(answer, 31);
    }
}
