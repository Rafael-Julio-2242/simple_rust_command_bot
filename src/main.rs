use teloxide::{ prelude::*, utils::command::BotCommands };

use dotenv::dotenv;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported: ")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "Handle a username and an age", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
    #[command(description = "Handle a sum of 2 numbers", parse_with = "split")]
    SumTwoNumbers { first_number: u32, second_number: u32 },
    #[command(description = "Remenber....")]
    Remenber,
}

#[tokio::main]
async fn main() {

    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting command bot....");


    let bot = Bot::from_env();

    Command::repl(bot, answer).await;

}


async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?
        },
        Command::UsernameAndAge { username, age } => {
            bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}")).await?
        },
        Command::SumTwoNumbers { first_number, second_number } => {
            let sum = first_number + second_number;
            bot.send_message(msg.chat.id, format!("The sum of {first_number} and {second_number} is {sum}")).await?
        },
        Command::Remenber => bot.send_message(msg.chat.id, "Jesus Loves you...").await?
    };

    Ok(())
}