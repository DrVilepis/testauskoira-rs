use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub async fn github(ctx: &Context, interaction: ApplicationCommandInteraction) {
    interaction
        .create_interaction_response(&ctx.http, |r| {
            r.interaction_response_data(|d| {
                d.content(
                    "Linkki github organisaatioon:\n<https://koira.testausserveri.fi/github/join>",
                )
            })
        })
        .await
        .unwrap();
}
