mod question1;
mod question2;
mod question3;
mod question4;

fn main() {
    question1::run();
    question2::run();
    question3::run();
    question4::run();
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_addition() {
        assert_eq!(1 + 1, 2);
    }
}
