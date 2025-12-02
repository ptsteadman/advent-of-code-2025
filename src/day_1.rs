use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn get_password(path: impl AsRef<Path>) -> io::Result<i32> {
    let file = File::open(path)?;
    let mut curr_num: i32 = 50;
    let mut res: i32 = 0;
    for line in io::BufReader::new(file).lines().map_while(Result::ok) {
        let letter = &line[0..1];
        let num: i32 = line[1..].parse().expect("not a number");
        match letter {
            "L" => curr_num = (curr_num - num) % 100,
            "R" => curr_num = (curr_num + num) % 100,
            _ => panic!("invalid letter: {letter:?}"),
        }
        println!("curr_num: {curr_num:?}");
        if curr_num == 0 {
            res += 1;
        }
    }
    return Ok(res);
}

#[cfg(test)]
mod tests {
    use super::*;
    use runfiles::{rlocation, Runfiles};

    #[test]
    fn test_simple() {
        let r = Runfiles::create().unwrap();
        let path =
            rlocation!(r, "_main/src/day_1_input_simple.txt").expect("Failed to locate runfile");
        println!("{path:?}");
        assert!(get_password(&path).is_ok());
        assert_eq!(get_password(&path).unwrap(), 3);
    }
}
