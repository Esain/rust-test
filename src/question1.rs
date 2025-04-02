// 从命令行读取一个整数 n（若读取失败或没有输入则默认 n = 5）。
// 打印从 1 到 n 的所有整数，每行一个。
// 若该整数可以被 3 整除，则在数字后面附加输出 "Fizz"；若可以被 5 整除，则附加输出 "Buzz"；若同时满足可以被 3 和 5 整除的情况，则输出 "FizzBuzz"。
use std::env;

fn fetch_args(args: &[String]) -> usize {
    // 解析参数，若失败则默认 n = 5
    match args.get(1) {
        Some(arg) => arg.parse().unwrap_or(5),
        None => 5,
    }
}


fn format_n(n :usize) -> String {
    if n % 3 == 0 && n % 5 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        n.to_string() + "Fizz"
        // println!("{} Fizz", i);
    } else if n % 5 == 0 {
        n.to_string() + "Buzz"
    } else {
        n.to_string()
    }
}
pub fn run() {
    // 从命令行读取参数
    let args: Vec<String> = env::args().collect();
    // 获取 n 的值 
    let n = fetch_args(&args);
    for i in 1..=n {
        println!("{}" , format_n(i));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let n = fetch_args(&vec![]);
        println!("non input");
        assert_eq!(n, 5);

        let n = fetch_args(&vec!["c".to_string(), "d".to_string()]);
        assert_eq!(n, 5);

        let n = fetch_args(&vec!["c".to_string(), 6.to_string()]);
        assert_eq!(n, 6);

    }

    #[test]
    fn test_println() {
        assert_eq!(format_n(1), "1");
        assert_eq!(format_n(3), "3Fizz");
        assert_eq!(format_n(5), "5Buzz");
        assert_eq!(format_n(15), "FizzBuzz");
    }
}

