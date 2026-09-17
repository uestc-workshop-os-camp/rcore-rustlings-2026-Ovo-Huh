// errors5.rs
//
// 这个程序使用了一个修改版的 errors4 代码。
//
// 这个练习涉及一些我们在课程后面才会学到的概念，比如 `Box` 和 `From` trait。现在不需要详细理解它们，
// 但如果你愿意可以提前阅读。暂时把 `Box<dyn ???>` 类型理解为“我想要任何实现 ??? 的东西”的类型，
// 考虑到 Rust 一贯的运行时安全标准，这种做法可能让你觉得有点宽松！
// 简而言之，使用 Box 的这个特定场景是：当你想拥有某个值，并且只在意它实现了某个特定 trait 时。为此，
// Box 被声明为 Box<dyn Trait> 类型，其中 Trait 是编译器在该上下文中会查找的 trait。对于这个练习，
// 这个上下文是可能在 Result 中返回的错误。
// 那么我们可以用什么来描述这两种错误呢？换句话说，是否有一个 trait 是这两种错误都实现的？
// Execute `rustlings hint errors5` or use the `hint` watch subcommand for a
// hint.


use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
