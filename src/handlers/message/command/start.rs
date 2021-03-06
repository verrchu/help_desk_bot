use crate::{greeting, Context};

use teloxide_core::{
    payloads::setters::*,
    requests::{Request, Requester},
    types::User,
};

pub async fn handle(user: &User, context: Context) -> anyhow::Result<()> {
    let tg = context.tg.clone();

    tracing::info!("processing command /start");

    let (header, keyboard) = greeting::render(context.lang);

    tg.send_message(user.id, header)
        .reply_markup(keyboard)
        .send()
        .await
        .map(|_| ())
        .map_err(anyhow::Error::from)?;

    Ok(())
}
