pub mod owner;
pub mod user;
pub mod driver_control;

pub mod register_user {
    use serenity::builder::CreateCommand;

    use crate::event_handler::Command;

    use super::user;
 
	pub fn register() -> Vec<CreateCommand> {
		return vec![
			user::Id.register(),
			user::Ping.register(),
			user::create_meeting::register(),

		]
	}
}

pub mod register_owner {
    use serenity::builder::CreateCommand;

    use super::owner;

	pub fn register() -> Vec<CreateCommand> {
		return vec![
			owner::slow_mode::register()
		]
	}
}

pub mod register_driver_control {
	use serenity::builder::CreateCommand;

	use super::driver_control;

	pub fn register() -> Vec<CreateCommand> {
		return vec![
			driver_control::join_channel::register(),
			driver_control::leave_channel::register(),
		]
	}
}
