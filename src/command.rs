use getset::{Getters, Setters};
use nom::bytes::complete::take_while1;
// 引入 nom 的宏和函数
#[allow(unused_imports)]
use nom::{branch::alt, bytes::complete::{tag, take_until}, character::complete::{alpha1, alphanumeric1, anychar, multispace1, space1}, multi::many1, sequence::{delimited, preceded, terminated}, IResult};

// 定义一个枚举类型来表示不同的命令
#[derive(Debug,PartialEq)]
pub enum Command {
    Login(UserPass),
    Send(TextToUser),
    Logout(String),
    Quit,
    Exit,
}

// 定义一个结构体来表示用户和密码的参数
#[derive(Debug, PartialEq,Getters, Setters)]
pub struct UserPass {
    #[getset(set = "pub", get = "pub")]
    user: String,
    #[getset(set = "pub", get = "pub")]
    pass: String,
}

// 定义一个结构体来表示文本和用户的参数
#[derive(Debug, PartialEq,Getters, Setters)]
pub struct TextToUser {
    #[getset(set = "pub", get = "pub")]
    text: String,
    #[getset(set = "pub", get = "pub")]
    user: String,
}

// 定义一个解析器来解析 login user pass 命令
fn parse_login(input: &str) -> IResult<&str, Command> {
    // 匹配 login 关键字和一个或多个空格
    let (input, _) = tag("login")(input)?;
    let (input, _) = space1(input)?;
    // 匹配一个或多个字母作为用户名
    // let (input, user) = alphanumeric1(input)?;
    let (input, user) = take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == '@' || c== '.' )(input)?;
    let user = user.to_string();
    // 匹配一个或多个空格
    let (input, _) = space1(input)?;
    // 匹配一个或多个字母或数字作为密码
    // let (input, pass) = alphanumeric1(input)?;
    let (input, pass) = take_while1(|c: char| c.is_alphanumeric() || "!@#$%^&*()_-+=<>?".contains(c))(input)?;
    let pass = pass.to_string();
    // 返回解析结果为 Login 类型的命令
    Ok((input, Command::Login(UserPass { user, pass })))
}

// 修改解析 send 命令的函数
fn parse_send(input: &str) -> IResult<&str, Command> {
    // 匹配 send 关键字和一个或多个空格
    let (input, _) = tag("send")(input)?;
    // println!("ctx: {:?}\n",ctx);
    let (input, _) = space1(input)?;
    // println!("ctx: {:?}\n",ctx);

    // 匹配一个或多个非空格字符作为文本，使用 preceded 和 terminated 来跳过左右两边的双引号
    // let (input, text) = preceded(tag("\""), terminated(many1(anychar), tag("\"")))(input)?;
    // let text = text.into_iter().collect();

    // 匹配一个或多个非空格字符作为文本，使用 delimited 和 take_until 来跳过左右两边的双引号
    let (input, text) = delimited(tag("\""), take_until("\""), tag("\""))(input)?;
    let text = text.to_string();

    // 匹配一个或多个空格和 to 关键字
    let (input, _) = space1(input)?;
    // println!("ctx: {:?}\n",ctx);
    let (input, _) = tag("to")(input)?;
    // println!("ctx: {:?}\n",ctx);

    // 匹配一个或多个空格和一个或多个字母作为用户，使用 preceded 和 terminated 来跳过左右两边的双引号
    let (user, _) = space1(input)?;
    // println!("ctx: {:?}\n",ctx);
    // let (input, user) = preceded(tag("\""), terminated(alpha1, tag("\"")))(input)?;
    let user = user.to_string();

    // 返回解析结果为 Send 类型的命令
    Ok((input, Command::Send(TextToUser { text, user })))
}

fn parse_logout(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("logout")(input)?;
    let (input, _) = space1(input)?;

    // 匹配用户名
    let (input, user) = take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == '@' || c == '.')(input)?;
    let user = user.to_string();

    Ok((input, Command::Logout(user)))
}

fn parse_exit(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("exit")(input)?;
    Ok((input, Command::Exit))
}

fn parse_quit(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("quit")(input)?;
    Ok((input, Command::Quit))
}

// 定义一个解析器来解析任意的命令，使用 alt 的宏形式来选择不同的解析器
pub fn parse_command(input: &str) -> IResult<&str, Command> {
    alt((parse_login, parse_send, parse_logout, parse_exit, parse_quit))(input)
}

// 测试一下我们的解析器
pub fn main() {
    // 一些测试用例
    let cases = vec![
        (
            "login alice 123",
            Command::Login(UserPass {
                user: "alice".to_string(),
                pass: "123".to_string(),
            }),
        ),
        (
            "send \"hello world!\" to bob",
            Command::Send(TextToUser {
                text: "hello".to_string(),
                user: "bob".to_string(),
            }),
        ),
        (
            "login bob 456",
            Command::Login(UserPass {
                user: "bob".to_string(),
                pass: "456".to_string(),
            }),
        ),
        (
            "send \"world 世界,你好! Hello!\" to alice",
            Command::Send(TextToUser {
                text: "world".to_string(),
                user: "alice".to_string(),
            }),
        ),
    ];
    // 对每个测试用例，调用 parse_command 解析器，打印输入和输出
    for (input, expected) in cases {
        println!("Input: {}", input);
        let result = parse_command(input);
        println!("Output: {:?}\n", result);
        // 检查输出是否和预期一致
        println!("input: {:?}\n", expected);
        println!("input: {:?}\n",result.err());
        // assert_eq!(result, Ok(("", expected)));
    }
}
