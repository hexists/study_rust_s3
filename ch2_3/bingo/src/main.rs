use rand::seq::SliceRandom;

fn main() {
    const NUMS_N:usize = 75;
    let mut nums = [0; NUMS_N];
    let mut rng = rand::thread_rng();

    for n in 0..NUMS_N {
        nums[n] = n + 1;
    }

    nums.shuffle(&mut rng);

    for y in 0..5 {
        for x in 0..5 {
            let index = y*5 + x;
            let mut val = "*";
            if index == 12 {
                print!("  *");
            }
            else {
                print!("{:3}", nums[index]);
            }
        }
        println!();
    }
}
