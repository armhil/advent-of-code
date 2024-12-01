use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn part1_calculate_difference(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
  let mut l1 = list1;
  let mut l2 = list2;
  l1.sort();
  l2.sort();

  let mut result = 0;
  for i in 0..l1.len() {
    result += (l1[i] - l2[i]).abs();
  }

  return result;
}


fn part2_calculate_similarity(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
  // build the dictionary
  let mut set: HashMap<i32, i32> = HashMap::new();
  for i in 0..list2.len() {
    match set.get(&list2[i]) {
      Some(&value) => set.insert(list2[i], &value+1),
      None => set.insert(list2[i], 1),
    };
  }

  let mut result = 0;
  for i in 0..list1.len() {
    match set.get(&list1[i]) {
      Some(&value) => result+= list1[i]*value,
      None => ()
    };
  }
  return result;
}
fn main() -> io::Result<()> {
  let file = File::open("index.txt")?;
  let reader = io::BufReader::new(file);
  let mut first_list: Vec<i32> = vec![];
  let mut second_list: Vec<i32> = vec![];
  for line in reader.lines() {
    let line = line?;
    let first_part= &line[0..5]; // Get the first 5 characters
    let second_part = &line[line.len()-5..line.len()];
    first_list.push(first_part.parse().unwrap());
    second_list.push(second_part.parse().unwrap());
  }

  let result = part2_calculate_similarity(first_list, second_list);

  println!("Total diff: {}", result);

  Ok(())
}
