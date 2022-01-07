use crate::Db;

use teloxide_core::{requests::Requester, types::CallbackQuery, Bot, RequestError};

pub async fn handle_callback_query<R: Requester<Err = RequestError>>(
    bot: R,
    update: &CallbackQuery,
    db: Db,
) -> anyhow::Result<()> {
    Ok(())
}
