use std::vec; 

pub struct Solution {}

impl Solution {
    fn str_match(index: usize, origin: &String, word: &String) -> bool {
        if index + word.len() -1 >= origin.len() {
            return false;
        } 

        for i in index..index+word.len() {
            if origin.as_bytes()[i] != word.as_bytes()[i-index] {
                return false;
            }
        }
        return true;
    }

    fn check(index: usize, origin: &Vec<Vec<usize>>, num: usize, gap: usize) -> bool {
        let mut set = vec![false; num];

        for i in 0..num {
            if index + i*gap >= origin.len() {
                return false;
            }
            if origin[index + i*gap].len() == 0 {
                return false;
            } else {
                let mut pass = false;
                for j in 0 .. origin[index + i*gap].len() {
                    if set[origin[index+i*gap][j]] == false {
                        set[origin[index+i*gap][j]] = true;
                        pass = true;
                        break;
                    }
                }
                if !pass {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn find_substring(self, s: String, words: Vec<String>) -> Vec<i32> {
        let mut res : Vec<i32> = Vec::new();
        let mut flag = vec![vec![]; s.len()];
        for i in 0..s.len() {
            for j in 0..words.len() {
                if Solution::str_match(i, &s, &words[j]) {
                    flag[i].push(j);
                }
            }
        }
        for i in 0..s.len() {
            if Solution::check(i, &flag, words.len(), words[0].len()) {
                res.push(i as i32);
            }
        }
        return res;
    }
}