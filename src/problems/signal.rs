use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

use super::Problem;

pub struct Signal {}

impl Problem for Signal {
    fn solve(&mut self, f: File) -> Result<(), Box<dyn Error>> {
        let mut data = String::new();
        BufReader::new(f).read_to_string(&mut data)?;
        println!("{}", start_of_packet(&data));
        println!("{}", start_of_message(&data));
        Ok(())
    }
}

fn start_of_packet(data: &str) -> usize {
    unique_prefix(data, 4)
}

fn start_of_message(data: &str) -> usize {
    unique_prefix(data, 14)
}

fn unique_prefix(data: &str, len: usize) -> usize {
    let mut ctrs: HashMap<u8, usize> = HashMap::new();
    for (i, c) in data.bytes().enumerate() {
        ctrs.insert(c, ctrs.get(&c).unwrap_or(&0) + 1);
        if i >= len {
            let ch = data.as_bytes()[i - len];
            ctrs.insert(ch, ctrs.get(&ch).unwrap() - 1);
            if ctrs[&ch] == 0 {
                ctrs.remove(&ch);
            }
        }
        if ctrs.len() == len {
            return i + 1;
        }
    }
    panic!("Couldn't find");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker() {
        assert_eq!(start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(start_of_packet("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);

        assert_eq!(start_of_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(start_of_message("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(start_of_message("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(start_of_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(start_of_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
