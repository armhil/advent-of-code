use std::fs::File;
use std::io;
use std::io::BufRead;

#[allow(unused_assignments)]
pub fn read_file() -> Vec<Vec<i32>> {
  let file = File::open("./src/day2/input.txt").unwrap();
  let reader = io::BufReader::new(file);
  let mut list: Vec<i32> = vec![];
  let mut result: Vec<Vec<i32>> = vec![];
  for line in reader.lines() {
    list = vec![];
    for text_number in line.unwrap().split_whitespace() {
      list.push(text_number.parse().unwrap());
    }
    result.push(list);
  }
  return result;
}

#[allow(unused_assignments)]
fn check_if_safe(input: Vec<i32>) -> bool {
  let mut direction = 0;
  if input[0] < input[1] {
    // increasing
    direction = 1;
  }
  else {
    direction = -1;
  }
  let mut is_valid = true;
  for i in 1..input.len() {
    let diff = (input[i-1]-input[i]).abs();
    if diff > 3 || diff < 1 {
      is_valid = false;
      break;
    }
    if direction == 1 && input[i] < input[i-1] {
      is_valid = false;
      break;
    }
    else if direction == -1 && input[i] > input[i-1] {
      is_valid = false;
      break;
    }
  }

  return is_valid;
}

#[allow(unused_assignments)]
pub fn count_safe_reports(input: Vec<Vec<i32>>) -> i32 {
  let mut result = 0;
  for i in 0..input.len() {
    if check_if_safe(input[i].clone()) {
      result += 1;
    }
    else {
      for j in 0..input[i].len() {
        let mut c = input[i].clone();
        c.remove(j);
        // this is not optimal
        if check_if_safe(c) {
          result += 1;
          break;
        }
      }
    }
  }

  return result;
}