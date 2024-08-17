use std::collections::HashMap;

fn main() {
    let res = tow_sum(vec![3,2,4], 6);
    println!("{:?}", res);
}

fn tow_sum(nums:Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i,v) in nums.iter().enumerate() {
        let rem = target - v;
        if let Some(j) = map.get(&rem) {
            return vec![i as i32,*j];
        }
        map.insert(v, i as i32);
    }
    vec![]
}
