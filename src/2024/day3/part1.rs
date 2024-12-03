use core::num;
use std::fs;
use std::io::{self, Read};
use regex::Regex;

fn calc_from_str(str: &str) -> i32 {
  let parts: Vec<&str> = str.split(',').collect();
  let part1 = &parts[0][4..];
  let part2len = parts[1].len();
  let part2 = &parts[1][..part2len-1];
  let n1: i32 = part1.parse().unwrap();
  let n2: i32 = part2.parse().unwrap();
  return n1*n2;
}

#[allow(unused_assignments)]
pub fn read_file_part1() -> i32 {
  let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
  let contents = fs::read_to_string("./src/day3/input.txt").unwrap();
  
  let matches: Vec<&str> = re.find_iter(&contents).map(|mat| mat.as_str()).collect();
  let mut sum = 0;
  for str in matches {
    sum += calc_from_str(&str);
  }
  println!("{}", sum);
  return sum;
}

#[allow(unused_assignments)]
pub fn read_file_part2() -> i32 {
  let num_regex: Regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
  let do_regex = Regex::new(r"do\(\)").unwrap();
  let dont_regex = Regex::new(r"don't\(\)").unwrap();
  let mut contents = fs::read_to_string("./src/day3/input.txt").unwrap();

  let mut indexOps: Vec<(usize, i32, i32)> = Vec::new();
  let num_matches: Vec<(usize, &str)> = num_regex.find_iter(&contents)
  .map(|mat| (mat.start(), mat.as_str()))
  .collect();
  let do_matches: Vec<usize> = do_regex.find_iter(&contents)
  .map(|mat | mat.start()).collect(); 
  let dont_matches: Vec<usize> = dont_regex.find_iter(&contents)
  .map(|mat | mat.start()).collect();
  for num_match in num_matches {
    indexOps.push((num_match.0, 0, calc_from_str(num_match.1)));
  }
  for do_match in do_matches {
    indexOps.push((do_match, 1, 0));
  }
  for dont_match in dont_matches {
    indexOps.push((dont_match, -1, 0))
  }
  indexOps.sort_by_key(|k | k.0);

  let mut sum = 0;
  let mut include = true;
  for entry in indexOps {
    if entry.1 == -1 {
      include = false;
    }
    else if entry.1 == 1 {
      include = true;
    }
    else {
      if include {
        sum += entry.2;
      }
    }
  }

  println!("{}", sum);
  return sum;
}