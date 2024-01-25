#![feature(allocator_api)]

mod structures;
mod util;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::structures::CommandFun;

/// `command` is a procedural macro that generates a module for a command in a Discord bot.
///
/// This macro takes two arguments:
/// * `_attr: TokenStream`: This argument is currently unused. It could be used for future enhancements, such as adding additional attributes to the command.
/// * `item: TokenStream`: This is the function that will be used to generate the command. The function's name is used as the command name, and the function's body is used as the body of the `run` function in the generated module.
///
/// The macro generates a module with the following structure:
///
/// ```rust
/// pub mod #fn_name {
///     use serenity::{ all::ResolvedOption, builder::CreateCommand };
///
///     pub fn register() -> CreateCommand {
///         CreateCommand::new(stringify!(#fn_name)).description("It pings!")
///     }
///
///     pub fn run(_options: &[ResolvedOption]) -> String {
///         #fn_body
///     }
/// }
/// ```
///
/// Where:
/// * `#fn_name` is the name of the input function.
/// * `#fn_body` is the body of the input function.
///
/// The `register` function creates a new command with the name of the input function and a default description "It pings!".
/// The `run` function has the same body as the input function.
///
/// # Examples
///
/// ```rust
/// #[command]
/// fn ping() -> String {
///     "Pong!".to_owned()
/// }
/// ```
///
/// This will generate the following module:
///
/// ```rust
/// pub mod ping {
///     use serenity::{ all::ResolvedOption, builder::CreateCommand };
///
///     pub fn register() -> CreateCommand {
///         CreateCommand::new("ping").description("It pings!")
///     }
///
///     pub fn run(_options: &[ResolvedOption]) -> String {
///         "Pong!".to_owned()
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn command(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // dbg!(_attr, input.clone());

    let input2 = input.clone();
    let func = parse_macro_input!(input2 as CommandFun);
    let imports = &func.imports;
    let name = &func.name;
    let visibility = &func.visibility;
    let body = &func.body;

    // let mut expanded = TokenStream2::new();
    // func.to_tokens(&mut expanded);

    // dbg!(&expanded);

    let expanded =
        quote! {
        #visibility mod #name {
            use serenity::{builder::CreateCommand, all::ResolvedOption};
            #(#imports)*
            #visibility async fn register() -> CreateCommand {
                // CreateCommand::new(stringify!(#fn_name)) #description
                CreateCommand::new(stringify!(#name)).description("It pings")
            }
            
            #visibility fn run(_options: &[ResolvedOption]) -> String {
                #(#body)*
                // #(#fn_body)*
            }
        }
    };

    TokenStream::from(expanded)

}
