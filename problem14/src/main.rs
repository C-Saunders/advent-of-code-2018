fn main() {
    part1();
}

fn part1() {
    let mut recipies = vec![3, 7];
    let mut cursor_one = 0;
    let mut cursor_two = 1;

    loop {
        if recipies.len() > 704321 + 10 {
            println!("{:?}", &recipies[704321..(704321 + 10)]);
            break;
        }

        let sum = recipies[cursor_one] + recipies[cursor_two];

        let sum_str = sum.to_string();
        let first_digit: usize = sum_str.get(0..1)
            .expect("Sum was empty")
            .parse()
            .expect("Failed to parse as integer");

        recipies.push(first_digit);

        let second_digit = sum_str.get(1..).expect("Sum was empty");
        if !second_digit.is_empty() {
            recipies.push(second_digit.parse::<usize>().expect("Failed to parse as integer"));
        }

        cursor_one = (cursor_one + 1 + recipies[cursor_one]) % recipies.len();
        cursor_two = (cursor_two + 1 + recipies[cursor_two]) % recipies.len();
    }
}
