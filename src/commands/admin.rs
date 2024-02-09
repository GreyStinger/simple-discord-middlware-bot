pub mod id {
    use serenity::all::GuildId;
    use serenity::builder::{ CreateCommand, CreateCommandOption };
    use serenity::model::application::{ CommandOptionType, ResolvedOption, ResolvedValue };
    use serenity::model::id::RoleId;
    use serenity::prelude::Context;

    pub fn register() -> CreateCommand {
        CreateCommand::new("id")
            .description("Get user ids")
            .add_option(
                CreateCommandOption::new(CommandOptionType::Role, "role", "The role to lookup")
                    .required(true)
            )
    }

    pub async fn run(ctx: &Context, guild_id: GuildId, options: &[ResolvedOption<'_>]) -> String {
        let mut response = String::new();

        for option in options {
            if let ResolvedOption { value: ResolvedValue::Role(role), .. } = option {
                let guild_id = guild_id;
                let members = guild_id.members(&ctx.http, None, None).await.unwrap();

                let role_id = RoleId::new(role.id.get());
                for member in members {
                    if member.roles.contains(&role_id) {
                        response.push_str(&format!("{}'s id is {}\n", member.user.tag(), member.user.id));
                    }
                }
            }
        }

        if response.is_empty() {
            return "Please provide a valid role".to_string();
        }
    
        response
    }
}