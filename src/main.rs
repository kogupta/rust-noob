use std::collections::HashMap;

mod rotate_sorted_array;

fn main() {
    println!("Hello, world!");
    println!("{:?}", two_sum(vec![3, 2, 4], 6));
    println!("{:?}", two_sum(vec![3, 3], 6));
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut kv: HashMap<i32, usize> = HashMap::new();

    for (index, value) in nums.iter().enumerate() {
        kv.insert(*value, index);
    };

    for (index, value) in nums.iter().enumerate() {
        let req = target - value;
        if let Some(idx) = kv.get(&req) {
            if *idx != index {
                return vec![index as i32, *idx as i32]
            }
        }
    }

    vec![]
}
