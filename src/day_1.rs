use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn get_password(path: impl AsRef<Path>, count_rotations: bool) -> io::Result<i32> {
    let file = File::open(path)?;
    let mut curr_num: i32 = 50;
    let mut rotations: i32 = 0;
    let mut res: i32 = 0;
    for line in io::BufReader::new(file).lines().map_while(Result::ok) {
        let letter = &line[0..1];
        let num: i32 = line[1..].parse().expect("not a number");
        match letter {
            "L" => {
                rotations = (curr_num - num).div_euclid(100).abs();
                curr_num = (curr_num - num).rem_euclid(100);
                // if curr_num == 0 && rotations == 0 && num > 0 {
                //     rotations = 1;
                // }
            }
            "R" => {
                rotations = (curr_num + num).div_euclid(100).abs();
                curr_num = (curr_num + num).rem_euclid(100);
            }
            _ => panic!("invalid letter: {letter:?}"),
        }
        println!("line: {line:?}");
        println!("curr_num: {curr_num:?}");
        println!("rotations: {rotations:?}");
        if count_rotations {
            res += rotations;
        } else if curr_num == 0 {
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
        assert_eq!(get_password(&path, false).unwrap(), 3);
        assert_eq!(get_password(&path, true).unwrap(), 6);
    }

    // #[test]
    // fn test() {
    //     let r = Runfiles::create().unwrap();
    //     let path = rlocation!(r, "_main/src/day_1_input.txt").expect("Failed to locate runfile");
    //     assert!(get_password(&path, false).is_ok());
    //     assert_eq!(get_password(&path, false).unwrap(), 1055);
    //     assert_eq!(get_password(&path, true).unwrap(), 1055);
    // }
}
