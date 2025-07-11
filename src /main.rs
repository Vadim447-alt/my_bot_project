

use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, CallbackQuery};
use teloxide::utils::command::BotCommand;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot.clone(), move |msg| {
        let bot = bot.clone();
        async move {
            if let Some(text) = msg.update.text() {
                match text {
                    "/start" => {
                        let keyboard = InlineKeyboardMarkup::new(vec![
                            vec![
                                InlineKeyboardButton::callback("💎 Купить Гемы", "buy_gems"),
                                InlineKeyboardButton::callback("🪙 Купить Робуксы", "buy_robux"),
                            ],
                        ]);
                        msg.answer("Выберите товар:").reply_markup(keyboard).await?;
                    }
                    _ => {
                        msg.answer("Пожалуйста, выберите товар с помощью кнопок.").await?;
                    }
                }
            }
            Ok(())
        }
    })
    .await;
}
