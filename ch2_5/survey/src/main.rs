use std::collections::HashMap;
use std::collections::BTreeMap;


fn main() {
    const VOTES:&str = "C,C,A,A,A,B,C,C,B,B,B,C,B,C,A,C,C,B,C,C,C";
    // let mut vote_counts = HashMap::new();
    let mut vote_counts = BTreeMap::new();

    for vote in VOTES.split(',') {
        // println!("{}", vote);

        if ! vote_counts.contains_key(vote) {
            vote_counts.insert(vote, 1);
        }
        else {
            vote_counts.insert(vote, vote_counts[vote] + 1);
        }
    }
    // println!("{:?}", vote_counts);

    for (vote, count) in &vote_counts {
        println!("{vote}: {count}");
    }
}
