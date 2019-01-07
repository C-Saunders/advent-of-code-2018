use std::fs;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;
    let as_array = get_as_array(&input);
    let ChildProcessResult { metadata_sum, .. } = part1(&as_array);
    assert_eq!(metadata_sum, 36891);
    println!("Metadata sum = {}",  metadata_sum);
    Ok(())
}

fn get_as_array(input: &str) -> Vec<usize> {
    input.split(' ').map(|item| item.parse::<usize>().unwrap()).collect()
}

#[derive(Debug, PartialEq, Eq)]
struct ChildProcessResult {
    total_length: usize,
    metadata_sum: usize,
}

fn part1(data: &[usize]) -> ChildProcessResult {
    let num_children = data[0];
    let num_metadata = data[1];

    let mut running_metadata_sum = 0;
    let mut running_len_total = 2;

    for _ in 0..num_children {
        let ChildProcessResult { total_length: additional_len, metadata_sum: additional_sum } = part1(&data[running_len_total..]);
        running_len_total += additional_len;
        running_metadata_sum += additional_sum;
    }

    let this_metadata_sum: usize = data[running_len_total..running_len_total + num_metadata].iter().sum();

    ChildProcessResult {
        total_length: running_len_total + num_metadata,
        metadata_sum: running_metadata_sum + this_metadata_sum,
    }
}

#[cfg(test)]
mod test_process_child {
    use super::{process_child, ChildProcessResult};

    #[test]
    fn no_children() {
        assert_eq!(
            ChildProcessResult{ total_length: 5, metadata_sum: 9 + 5 + 12 },
            process_child(&[0, 3, 9, 5, 12]),
        )
    }

    #[test]
    fn one_child() {
        assert_eq!(
            ChildProcessResult{ total_length: 7, metadata_sum: 9 + 5 + 12 },
            process_child(&[1, 2, 0, 1, 9, 5, 12]),
        )
    }

    #[test]
    fn sub_children() {
        assert_eq!(
            ChildProcessResult{ total_length: 16, metadata_sum: 1 + 1 + 2 + 10 + 11 + 12 + 2 + 99 },
            process_child(&[2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]),
        )
    }

    #[test]
    fn branched_children() {
        assert_eq!(
            ChildProcessResult{ total_length: 16, metadata_sum: 1 + 1 + 6 + 8 + 4 + 0 + 2 + 9 },
            process_child(&[2, 3, 1, 1, 0, 3, 1, 1, 6, 8, 0, 1, 4, 0, 2, 9]),
            //             [A, A, B, B, C, C, c, c, c, b, D, D, d, a, a, a]
        )
    }
}