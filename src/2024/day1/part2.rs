use std::collections::HashMap;

pub fn calculate_similarity(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
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