pub mod id {
    use serenity::builder::{ CreateCommand, CreateCommandOption };
    use serenity::model::application::{ CommandOptionType, ResolvedOption, ResolvedValue };

    pub fn register() -> CreateCommand {
        CreateCommand::new("id")
            .description("Get a user id")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::User,
                    "id",
                    "The user to lookup"
                ).required(true)
            )
    }
 
	pub fn run(options: &[ResolvedOption]) -> String {
		// if let Some(ResolvedOption { value: ResolvedValue::User(user, _), .. }) = options.first() {
        //     format!("{}'s id is {}", user.tag(), user.id)
        // } else {
        //     "Please provide a valid user".to_string()
        // }

		let mut response = String::new();
	
		for option in options {
			if let ResolvedOption { value: ResolvedValue::Autocomplete { kind: CommandOptionType::SubCommand, value: "Users" }, name: "id" , ..} = option {
				// response.push_str(&format!("{}'s id is {}\n", user.tag(), user.id));
			}
		}
	
		if response.is_empty() {
			response = "Please provide a valid user".to_string();
		}
	
		response
	}
}
