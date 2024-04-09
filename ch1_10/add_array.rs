fn main() {
    let nums = [1,2,3,4,5,6,7,8,9,10];
    let mut acc = 0;
    for n in 0..nums.len() {
        acc += nums[n];
    }
    println!("{}", acc);
}
