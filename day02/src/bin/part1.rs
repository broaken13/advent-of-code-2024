fn main() {
    println!("Part 1!");
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
    let mut numbers = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());

    let mut previous = numbers.next().unwrap();
    let mut num = numbers.next().unwrap();

    let is_increasing = num > previous;

    if is_increasing {
        if num <= previous || num > previous + 3 {
            return false;
        }
    } else {
        if num >= previous || num < previous - 3 {
            return false;
        }
    }

    for next_num in numbers {
        previous = num;
        num = next_num;

        if is_increasing {
            if num <= previous || num > previous + 3 {
                return false;
            }
        } else {
            if num >= previous || num < previous - 3 {
                return false;
            }
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn basic_test() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let answer = solution(input);
        assert_eq!(answer, 2);
    }
}
