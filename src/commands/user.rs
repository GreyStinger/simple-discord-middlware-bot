use greys_macros::command;

// pub mod ping {
//     use serenity::{ all::ResolvedOption, builder::CreateCommand };

//     pub fn register() -> CreateCommand {
//         CreateCommand::new("ping").description("It pings!")
//     }

//     pub fn run(_options: &[ResolvedOption]) -> String {
//         "Pong!".to_owned()
//     }
// }

// #[description("It Pings")]
#[command]
pub fn ping() -> String {
    "Pong!".to_owned()
}

pub mod create_meeting {
    use serenity::{builder::CreateCommand, all::{ResolvedOption, ResolvedValue, CommandOptionType}};

    pub fn register() -> CreateCommand {
        CreateCommand::new("create_meeting").description(
            "Creates a meeting room for and notifies the requested users of said room"
        )
    }

    pub fn run(_options: &[ResolvedOption]) -> String {
		for option in _options {
			if let ResolvedOption { value: ResolvedValue::Autocomplete { kind: CommandOptionType::SubCommand, value: "Users" }, name: "id" , ..} = option {
			}
		}

        String::from("Ran create_meeting successfully")
    }
}

