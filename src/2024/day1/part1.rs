pub fn calculate_difference(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
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