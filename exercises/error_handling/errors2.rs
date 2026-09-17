// errors2.rs
//
// 假设我们正在写一个游戏，在游戏里你可以用代币购买物品。所有物品都要5个代币，
// 而且每次购买物品都会收取1个代币的处理费。游戏玩家会输入他们想买多少物品，然后 `total_cost`
// 函数会计算代币的总花费。不过，因为玩家输入的是数量，我们得到的是一个字符串——而且他们可能输入的根本不是数字！
// 现在，这个函数根本没有处理错误情况（成功情况也没有正确处理）。我们想做的是：
// 如果我们对一个非数字字符串调用 `parse` 函数，这个函数会返回一个 `ParseIntError`，
// 在这种情况下，我们希望立即从我们的函数返回这个错误，而不是去做乘法和加法
// 实现这一点至少有两种正确的方式——但是有一种要短得多！
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.
use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
