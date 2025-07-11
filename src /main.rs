

use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, CallbackQuery};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct Order {
    product: String,
    status: String, // например, "awaiting_payment", "awaiting_nick", "completed"
}

type Orders = Arc<Mutex<HashMap<i64, Order>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();
    let orders: Orders = Arc::new(Mutex::new(HashMap::new()));

    teloxide::repl(bot.clone(), move |bot: Bot, msg: UpdateWithCx<Bot, Message>| {
        let bot = bot.clone();
        let orders = orders.clone();

        async move {
            if let Some(text) = msg.update.text() {
                let chat_id = msg.chat_id();

                if text == "/start" {
                    let keyboard = InlineKeyboardMarkup::new(vec![
                        vec![
                            InlineKeyboardButton::callback("💎 Гемы", "buy_gems"),
                            InlineKeyboardButton::callback("🪙 Робуксы", "buy_robux"),
                        ],
                    ]);
                    bot.send_message(chat_id, "Привет! Что хочешь купить?")
                        .reply_markup(keyboard)
                        .await?;
                } else {
                    // Если пользователь пишет ник после выбора товара и инструкции
                    let mut orders_lock = orders.lock().unwrap();
                    if let Some(order) = orders_lock.get_mut(&chat_id) {
                        if order.status == "awaiting_nick" {
                            let nick = text;
                            order.status = "completed";

                            bot.send_message(chat_id, format!("Спасибо! Твой ник '{}' записан. Начинаю выдачу {}...", nick, order.product))
                                .await?;

                            // Здесь логика автоматической выдачи товара по нику
                            // Пока просто ответим, что всё сделано
                            bot.send_message(chat_id, format!("{} успешно выданы на аккаунт '{}'.", order.product, nick))
                                .await?;
                        } else {
                            bot.send_message(chat_id, "Пожалуйста, нажми /start чтобы начать.")
                                .await?;
                        }
                    } else {
                        bot.send_message(chat_id, "Пожалуйста, нажми /start чтобы начать.")
                            .await?;
                    }
                }
            }
            respond(())
        }
    })
    .await;

    Ok(())
}

teloxide::dispatch_with_listener(bot.clone(), move |rx| {
    let orders = orders.clone();
    async move {
        while let Some(update) = rx.recv().await {
            if let UpdateKind::CallbackQuery(Some(cq)) = update.kind {
                let chat_id = cq.from.id.0 as i64;
                let data = cq.data.unwrap_or_default();

                match data.as_str() {
                    "buy_gems" | "buy_robux" => {
                        let product = if data == "buy_gems" { "Гемы" } else { "Робуксы" };

                        {
                            let mut orders_lock = orders.lock().unwrap();
                            orders_lock.insert(chat_id, Order { product: product.to_string(), status: "awaiting_nick".to_string() });
                        }

                        bot.answer_callback_query(&cq.id)
                            .text(format!("Вы выбрали {}! Оплатите сумму X через выбранный метод и отправьте мне свой игровой ник.", product))
                            .await?;

                        bot.send_message(cq.from.id, "После оплаты пришлите мне ваш игровой ник для начисления валюты.")
                            .await?;
                    }
                    _ => {}
                }
            }
        }
    }
})
.await;
