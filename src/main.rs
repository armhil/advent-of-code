use std::io;

mod day3;

fn main() {
  /*let file = File::open("./src/day2/input.txt")?;
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

  let result = day1::part2::part2_calculate_similarity(first_list, second_list);

  println!("Total diff: {}", result);
*/
  day3::part1::read_file_part2();

}
