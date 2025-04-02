// 从命令行参数接收一个文件路径，例如 input.txt。如果没有提供路径或文件无法打开，给出相应的错误提示并退出。
// 读取该文件的所有文本内容，统计文本中一共包含的字符数（不含换行符）与行数，并将结果写入 output.txt。
// 若 output.txt 文件已存在，可以选择直接覆盖或者追加，任选其一，但需要在程序里明确注释或说明处理方式。
use std::env;
use std::fs::{self, OpenOptions};

use std::io::Write;


fn main() {
    // 从命令行参数接收一个文件路径
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("错误: 请提供一个文件路径作为参数。");
        std::process::exit(1);
    }

    let input_file_path = &args[1];

    // 尝试打开并读取文件内容
    let content = match fs::read_to_string(input_file_path) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("错误: 无法打开文件 {}。", input_file_path);
            std::process::exit(1);
        }
    };

    // 统计字符数（不含换行符）与行数
    let char_count = content.chars().filter(|&c| c != '\n').count();
    let line_count = content.lines().count();

    // 准备写入 output.txt 的内容
    let output_content = format!("字符数: {}\n行数: {}\n", char_count, line_count);

    // 将结果写入 output.txt
    let output_file_path = "output.txt";
    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // 直接覆盖文件内容
        .open(output_file_path)
    {
        Ok(file) => file,
        Err(_) => {
            eprintln!("错误: 无法写入文件 {}。", output_file_path);
            std::process::exit(1);
        }
    };

    if let Err(e) = file.write_all(output_content.as_bytes()) {
        eprintln!("错误: 写入文件时发生错误: {}", e);
        std::process::exit(1);
    }

    println!("统计结果已写入 {}。", output_file_path);

}
#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, process::Command};

    #[test]
    fn test_question4() {
        // 创建临时输入文件
        let input_file_path = "test_input.txt";
        let mut input_file = File::create(input_file_path).expect("无法创建测试输入文件");
        writeln!(input_file, "Hello\nRust\nWorld").expect("无法写入测试输入文件");

        // 运行主程序
        let output = Command::new("cargo")
            .args(&["run", "--bin", "rust-test", input_file_path])
            .output()
            .expect("无法运行程序");

        // 检查程序是否成功运行
        assert!(
            output.status.success(),
            "程序运行失败: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        // 验证输出文件内容
        let output_file_path = "output.txt";
        let output_content = fs::read_to_string(output_file_path).expect("无法读取输出文件");
        assert!(output_content.contains("字符数: 15"));
        assert!(output_content.contains("行数: 3"));

        // 清理测试文件
        fs::remove_file(input_file_path).expect("无法删除测试输入文件");
        fs::remove_file(output_file_path).expect("无法删除测试输出文件");
    }
}