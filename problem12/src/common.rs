pub fn str_to_numbers(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|ch| match ch {
            '.' => 0,
            '#' => 1,
            _ => panic!("Unknown character in input"),
        })
        .collect()
}
