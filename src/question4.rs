// 从命令行参数接收一个文件路径，例如 input.txt。如果没有提供路径或文件无法打开，给出相应的错误提示并退出。
// 读取该文件的所有文本内容，统计文本中一共包含的字符数（不含换行符）与行数，并将结果写入 output.txt。
// 若 output.txt 文件已存在，可以选择直接覆盖或者追加，任选其一，但需要在程序里明确注释或说明处理方式。

use std::io::{self, BufRead};

fn process_input(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.to_string());
    }
    result

}

pub fn run() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let input = reader.lines().next().unwrap().unwrap();

}