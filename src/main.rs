mod T1345;

fn main() {
    let solution = T1345::Solution{};
    let array = vec![7,7,2,1,7,7,7,3,4,1];
    println!("res: {}", solution.min_jumps(array));
}
