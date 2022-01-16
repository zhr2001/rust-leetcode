mod T1345;
mod T30;

fn main() {
    let solution1345 = T1345::Solution{};
    let array = vec![7,7,2,1,7,7,7,3,4,1];
    println!("res: {}", solution1345.min_jumps(array));

    let solution30 = T30::Solution{};
    let words:Vec<String> = vec!["word".to_string(),"good".to_string(),"best".to_string(),"good".to_string()];
    let s = "wordgoodgoodgoodbestword".to_string();
    println!("res: {:?}", solution30.find_substring(s, words));
}
