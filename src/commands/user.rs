use greys_macros::slash_command;

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
#[slash_command]
#[description("It Pings")]
pub fn ping(_options: &[ResolvedOption]) -> String {
    "Pong!".to_owned()
}

#[slash_command]
#[description("Creates a meeting room for and notifies the requested users of said room")]
pub fn create_meeting(options: &[ResolvedOption]) -> String {
        use serenity::all::{ResolvedValue, CommandOptionType};
		for option in options {
			if let ResolvedOption { value: ResolvedValue::Autocomplete { kind: CommandOptionType::SubCommand, value: "Users" }, name: "id" , ..} = option {
			}
		}

        String::from("Ran create_meeting successfully")
}

// pub mod create_meeting {
//     use greys_macros::slash_command;
//     use serenity::{builder::CreateCommand, all::{ResolvedOption, ResolvedValue, CommandOptionType}};

//     pub fn register() -> CreateCommand {
//         CreateCommand::new("create_meeting").description(
//             "Creates a meeting room for and notifies the requested users of said room"
//         )
//     }

//     pub fn run(_options: &[ResolvedOption]) -> String {
// 		for option in _options {
// 			if let ResolvedOption { value: ResolvedValue::Autocomplete { kind: CommandOptionType::SubCommand, value: "Users" }, name: "id" , ..} = option {
// 			}
// 		}

//         String::from("Ran create_meeting successfully")
//     }
// }

