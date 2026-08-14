use serde_json::{Value, json};
use tgrs::*;

fn callback_button(text: impl Into<String>) -> InlineKeyboardButton {
    let text = text.into();
    InlineKeyboardButton {
        callback_data: Some(text.clone()),
        text,
        style: None,
        copy_text: None,
    }
}

#[test]
fn response_discrimination_uses_ok_not_the_result_shape() {
    let response: Response = serde_json::from_value(json!({
        "ok": true,
        "result": true,
    }))
    .unwrap();
    assert!(matches!(
        response,
        Response::Ok(OkResponse {
            result: TelegramResult::Bool(true),
        }),
    ));

    let response: Response = serde_json::from_value(json!({
        "ok": false,
        "result": true,
        "error_code": 400,
        "description": "Bad Request",
    }))
    .unwrap();

    match response {
        Response::Err(error) => {
            assert_eq!(error.error_code, Some(400));
            assert_eq!(error.description.as_deref(), Some("Bad Request"));
        }
        Response::Ok(_) => panic!("a response with ok=false was classified as successful"),
    }
}

#[test]
fn unknown_successful_results_are_decode_errors() {
    assert!(
        serde_json::from_value::<Response>(json!({
            "ok": true,
            "result": [],
        }))
        .is_err(),
    );
}

#[test]
fn reply_markup_has_the_expected_json_shape() {
    let button = InlineKeyboardButton {
        text: "Delete".into(),
        callback_data: Some("delete".into()),
        style: Some(ButtonStyle::Danger),
        copy_text: None,
    };
    let markup = ReplyMarkup::from(InlineKeyboardMarkup::row([button]));

    assert_eq!(
        serde_json::to_value(markup).unwrap(),
        json!({
            "inline_keyboard": [[{
                "text": "Delete",
                "callback_data": "delete",
                "style": "danger",
            }]],
        }),
    );
}

#[test]
fn rows_groups_buttons_without_dropping_or_reordering_them() {
    let markup = InlineKeyboardMarkup::rows((1..=5).map(|n| callback_button(n.to_string())), 2);
    let labels: Vec<Vec<_>> = markup
        .inline_keyboard
        .into_iter()
        .map(|row| row.into_iter().map(|button| button.text).collect())
        .collect();

    assert_eq!(labels, vec![vec!["1", "2"], vec!["3", "4"], vec!["5"]],);
}

#[test]
fn secrets_are_redacted_from_debug_output_but_still_serialized() {
    const BOT_TOKEN: &str = "123456:bot-token";
    const WEBHOOK_SECRET: &str = "webhook-secret";

    let telegram = Telegram::new(BOT_TOKEN).unwrap();
    let webhook = SetWebhook {
        url: "https://example.com/webhook".into(),
        max_connections: None,
        allowed_updates: None,
        drop_pending_updates: None,
        secret_token: Some(WEBHOOK_SECRET.into()),
    };

    let telegram_debug = format!("{telegram:?}");
    let webhook_debug = format!("{webhook:?}");
    assert!(!telegram_debug.contains(BOT_TOKEN));
    assert!(!webhook_debug.contains(WEBHOOK_SECRET));
    assert!(telegram_debug.contains("[REDACTED]"));
    assert!(webhook_debug.contains("[REDACTED]"));

    let serialized = serde_json::to_value(webhook).unwrap();
    assert_eq!(serialized["secret_token"], Value::from(WEBHOOK_SECRET));
}
