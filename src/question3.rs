// 请从命令行读取一行字符串（例如 "apple banana pear banana apple banana"）。
// 使用空格进行拆分，统计每个单词出现的次数，并以从高到底的顺序输出。
// 如果出现次数相同，按单词本身的字典序从小到大排序输出。
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::cmp::Ordering;

// struct Fruit {
//     name: String,
//     count: u8,
// }

// impl Fruit {
//     fn new(name: &str, count: u8) -> Fruit {
//         Fruit {
//             name: name.to_string(),
//             count,
//         }
//     }
// }

fn process_lines<T: BufRead + Sized>(reader: T) -> String {
    reader.lines().next().unwrap().unwrap()
}

fn get_word_map (word: &Vec<&str>) -> HashMap<String, u8> {
    let mut word_count: HashMap<String, u8> = HashMap::new();
    for word in word {
        word_count.entry(String::from(*word)).and_modify(|counter| *counter += 1).or_insert(1);
    }
    word_count
}

// fn ordering(word_count_vec: &mut Vec<(String, u8)>){
fn ordering(word_count: HashMap<String, u8>) -> Vec<(String, u8)>{
    let mut word_count_vec: Vec<(String, u8)> = word_count.into_iter().collect();
    // 按照出现次数从高到低排序，如果出现次数相同，则按单词本身的字典序从小到大排序
    word_count_vec.sort_by(|a, b| {
        match b.1.cmp(&a.1) {
            Ordering::Equal => a.0.cmp(&b.0),
            other => other,
        }
    });
    word_count_vec
}
pub fn run() {
    // 从标准输入读取一行字符串
    let stdin = io::stdin();
    let reader = stdin.lock();
    let input =process_lines(reader);

    // 使用空格拆分字符串
    let words: Vec<&str> = input.split_whitespace().collect();

    // 统计每个单词出现的次数
    let word_count: HashMap<String, u8> = get_word_map(&words);
    
    let  word_count_vec = ordering(word_count);
    // 打印结果
    for (word, count) in &word_count_vec {
        println!("{}: {}", word, count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines() {
        let input = "apple banana pear banana apple banana";
        let reader = input.as_bytes();
        let result = process_lines(reader);
        assert_eq!(result, "apple banana pear banana apple banana");
    }

    #[test]
    fn test_get_word() {
        let words = vec!["apple", "banana", "pear", "banana", "apple", "banana"];
        let word_count = get_word_map(&words);
        assert_eq!(word_count.get("apple"), Some(&2));
        assert_eq!(word_count.get("banana"), Some(&3));
        assert_eq!(word_count.get("pear"), Some(&1));

       let word_count_vec = ordering(word_count);
        assert_eq!(word_count_vec[0], ("banana".to_string(), 3));
        assert_eq!(word_count_vec[1], ("apple".to_string(), 2));
        assert_eq!(word_count_vec[2], ("pear".to_string(), 1));
    }
}