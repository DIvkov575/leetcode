use std::collections::HashMap;

fn main() {
    // let a: HashMap<usize, usize> = HashMap::from_iter(vec![(1,1),(2,1),(3,1)]);
    // let b: HashMap<usize, usize> = HashMap::from_iter(vec![(2,1),(2,1),(3,1)]);
    // println!("{:?}", a==b);


    let a = vec![3,7,9];
    let b = vec![3,7,11];

    let c = can_be_equal(a, b);
    println!("{}", c);

}

pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
    let mut t_map: HashMap<i32, u16> = HashMap::with_capacity(target.len()/2);
    let mut a_map: HashMap<i32, u16> = HashMap::with_capacity(target.len()/2);

    for i in 0..target.len() {
        if a_map.contains_key(&arr[i]) {
            *a_map.get_mut(&arr[i]).unwrap() += 1;
        } else {
            a_map.insert(arr[i], 1);
        }

        if t_map.contains_key(&target[i]) {
            *t_map.get_mut(&target[i]).unwrap() += 1;
        } else {
            t_map.insert(target[i], 1);
        }
    }

    if t_map == a_map {
        true
    } else {
        false
    }
}
