use poise::CreateReply;
use serenity::all::CreateMessage;
use tokio::time::Instant;

use crate::commands::tags::tag_utils::get_data_and_id;
use crate::commands::utils::create_error_embed;
use crate::{Context, Error};

#[poise::command(prefix_command)]
pub async fn dtag(
	ctx: Context<'_>,
	#[description = "Tag name"] name: String,
) -> Result<(), Error> {
	let start = Instant::now();
	let msg = match ctx {
		| Context::Prefix(prefix_ctx) => Some(prefix_ctx.msg),
		| _ => None,
	};

	let (data, id) = get_data_and_id(ctx).await?;

	if let Ok(Some((_name, content))) = data.tag_db.get_tag(&name, id).await {
		let mut message = CreateMessage::default().content(content);

		if let Some(referenced_message) = msg.and_then(|m| m.message_reference.clone()) {
			message = message.reference_message(referenced_message);
		}

		ctx.channel_id()
			.send_message(ctx.serenity_context(), message)
			.await?;
	} else {
		ctx.send(CreateReply::default().embed(create_error_embed(&format!(
			"❌ Tag `{}` does not exist",
			name
		))))
		.await?;
	}

	if let Some(msg) = msg {
		msg.delete(ctx.serenity_context()).await?;
	}

	println!("dtag took {} ms", start.elapsed().as_millis());
	Ok(())
}
