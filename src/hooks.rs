use serenity::{ framework::standard::macros::hook, client::Context, all::Message };

use crate::CommandCounter;

/// This is a hook function that gets called before a command is processed.
///
/// # Arguments
///
/// * `ctx` - A reference to the context in which this command is being called.
/// * `msg` - A reference to the message that triggered this command.
/// * `command_name` - The name of the command that was triggered.
///
/// # Description
///
/// This function is called before a command is processed. It logs the command name and the author's name,
/// and increments a counter for the command in the shared state (`ctx.data`).
///
/// The counter is stored in a `CommandCounter` which is expected to be present in `ctx.data`.
///
/// If this function returns `false`, the processing of the command is halted.
///
/// # Returns
///
/// This function returns a boolean. If `true`, the command will be processed. If `false`, the command will not be processed.
///
/// # Examples
///
/// ```no_run
/// #[hook]
/// async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
///     println!("Got command '{}' by user '{}'", command_name, msg.author.name);
///
///     let mut data = ctx.data.write().await;
///     let counter = data.get_mut::<CommandCounter>().expect("Expected CommandCounter in TypeMap.");
///     let entry = counter.entry(command_name.to_string()).or_insert(0);
///     *entry += 1;
///
///     true // if `before` returns false, command processing doesn't happen.
/// }
/// ```
#[hook]
pub async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!("Got command '{}' by user '{}'", command_name, msg.author.name);

    let mut data = ctx.data.write().await;
    let counter = data.get_mut::<CommandCounter>().expect("Expected CommandCounter in TypeMap.");
    let entry = counter.entry(command_name.to_string()).or_insert(0);
    *entry += 1;

    true // if `before` returns false, command processing doesn't happen.
}
