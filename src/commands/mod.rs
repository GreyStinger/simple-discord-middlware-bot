pub mod owner;
pub mod user;
pub mod admin;

pub mod voice_client_control;

pub mod help {
    use serenity::builder::CreateCommand;

	pub fn register() -> CreateCommand {
		CreateCommand::new("help").description("Provides help popup")
	}
}
