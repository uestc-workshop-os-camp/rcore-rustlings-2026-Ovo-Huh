// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

// 步骤：
// 1. 如果提供的字符串长度为 0，应返回一个错误
// 2. 根据字符串中的逗号进行分割
// 3. 分割后应只返回 2 个元素，否则返回错误
// 4. 从分割结果中提取第一个元素作为名字
// 5. 从分割结果中提取另一个元素，并用类似 `"4".parse::<usize>()` 的方式解析为年龄
// 6. 如果在提取名字和年龄时出现问题，应返回错误
// 如果一切正常，则返回一个 Person 对象的 Result

// 另外：`Box<dyn Error>` 实现了 `From<&'_ str>`。这意味着如果想返回一个字符串错误信息，可以直接使用
// return `Err("my error message".into())`。

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty(){return Err(ParsePersonError::Empty)};
        let mut data = s.split(',');
        let first = match data.next(){
            Some(x) => x.to_string(),
            None => return Err(ParsePersonError::NoName),
        };
        if first.is_empty(){return Err(ParsePersonError::NoName)};

        let second = match data.next(){
            Some(x) => x.parse::<usize>(),
            None => return Err(ParsePersonError::BadLen),
        };
        if data.next().is_some(){return Err(ParsePersonError::BadLen)};

        match second{
            Err(e) => Err(ParsePersonError::ParseInt(e)),
            Ok(x) =>Ok(Person{
                name:first,
                age:x
            })
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
