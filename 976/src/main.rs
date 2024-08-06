fn main() {
    let a = vec![1,2,3,4,5];
    let b = largest_perimeter(a);
    println!("{}", b);
}

pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
    let mut sides = nums.clone();
    sides.sort_by(|a,b| b.cmp(&a));

    let mut idxs = [1,2,3];
    let mut max_idx = 2;

    while sides[idxs[0]] >= sides[idxs[1]] + sides[idxs[2]] {
        max_idx += 1;
        idxs[0] = idxs[1];
        idxs[1] = idxs[2];
        idxs[2] = max_idx
    }


    sides[idxs[0]] + sides[idxs[1]] + sides[idxs[2]]
}

