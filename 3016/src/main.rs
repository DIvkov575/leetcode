use std::cmp::Ordering;

fn main() {
    let a = "asdfa".to_string();
    let b = minimum_pushes(a);
    println!("{:?}", b);
}

pub fn minimum_pushes(word: String) -> i32 {
    let mut map: std::collections::HashMap<char, i32>= std::collections::HashMap::with_capacity(26);
    for s in word.chars() { *map.entry(s).or_insert(0i32) += 1 };

    let mut sum = 0;

    let mut map_arr = map.values().map(|x| x.to_owned()).collect::<Vec<i32>>();
    map_arr.sort_by(|b, d| d.cmp(&b));

    for (idx, count) in map_arr.iter().enumerate() {
        if idx/8 == 0 {
            sum += count;
        } else if idx/8 == 1 {
            sum += 2*count;
        } else if idx/8 == 2 {
            sum += 3*count;
        } else if idx/8 == 1 {
            sum += 4*count;
        }
    }

    sum
}