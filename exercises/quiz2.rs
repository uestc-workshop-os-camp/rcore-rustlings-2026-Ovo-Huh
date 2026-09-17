// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// 让我们用一个函数来构建一个小机器。作为输入，我们会给一个字符串和命令的列表。这些命令决定了将对字符串执行什么操作。它可以是：
// - 将字符串转为大写
// - 修剪字符串
// - 在字符串末尾添加指定次数的“bar”
// 具体形式如下：
// - 输入是一个长度为2的元组的向量，
//   第一个元素是字符串，第二个元素是命令。
// - 输出是一个字符串向量。
//
// No hints this time!
pub enum Command {
    Uppercase,  // - 将字符串转为大写
    Trim,       // - 修剪字符串 
    Append(usize),// - 在字符串末尾添加指定次数的“bar”
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String,Command)>) ->Vec<String>  {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            match command{
                Command::Uppercase =>{output.push(string.to_uppercase().to_string())},
                Command::Trim =>{output.push(string.trim().to_string())},
                Command::Append(x) =>{
                    // string.to_string().push_str(&"bar".repeat(*x));
                    // output.push(string);
                    let mut str:String = string.to_string();
                    str.push_str(&"bar".repeat(*x));
                    output.push(str);
            },
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
