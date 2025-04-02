// 定义一个 Student 结构体，包含以下字段：name、age、score
// 实现以下功能：
// - new(name: &str, age: u8, score: f32) -> Student：返回一个新的学生实例。
// - show(&self)：打印 Student 的信息，格式如 Name: Alice, Age: 18, Score: 95.5。
// - is_passed(&self) -> bool：如果 score >= 60.0 则返回 true，否则返回 false。

struct Student {
    name: String,
    age: u8,
    score: f32,
}
impl Student {
    // 创建一个新的学生实例
    fn new(name: &str, age: u8, score: f32) -> Student {
        Student {
            name: name.to_string(),
            age,
            score,
        }
    }

    // 打印学生信息
    fn show(&self) {
        println!("Name: {}, Age: {}, Score: {}", self.name, self.age, self.score);
    }

    // 判断是否及格
    fn is_passed(&self) -> bool {
        self.score >= 60.0f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student() {
        let mut student = Student::new("Alice", 18, 95.5);
        student.show();
        assert_eq!(student.is_passed(), true);

        student.score = 55.0;
        assert_eq!(student.is_passed(), false);
    }
}