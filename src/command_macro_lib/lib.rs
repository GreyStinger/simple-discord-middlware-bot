#![feature(allocator_api)]

pub(crate) mod structures;
pub(crate) mod attributes;
#[macro_use]
pub(crate) mod util;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Lit};

use structures::*;
use util::*;

#[proc_macro_attribute]
pub fn slash_command(_attr: TokenStream, input: TokenStream) -> TokenStream {

    let func = parse_macro_input!(input as CommandFun);

    // let imports = &func.imports;
    let name = &func.name;
    let visibility = &func.visibility;
    let args = &func.args;
    let body = &func.body;

    let mut description = None;

    for attr in &func.attrs {
        if is_rustfmt_or_clippy_attr(&attr.path()) {
            continue;
        }


        let values = propagate_err!(parse_values(attr));
        let _span = values.span;

        let name = values.name.to_string();
        let name = &name[..];

        match name {
            "description" => if let Some(Lit::Str(lit_str)) = values.literals.get(0) {
                description = Some(lit_str.value());
            }
            _ => {}
        }

        // TODO: Work on extracting and assigning attrs based on the enum
    }

    let description = description.map_or(quote!(), |desc| quote!(.description(#desc)));

    let expanded =
        quote! {
        #visibility mod #name {
            use serenity::{builder::CreateCommand, all::ResolvedOption};
            // #(#imports)*
            #visibility fn register() -> CreateCommand {
                // If attribute description exists put string value in .description(<String>)
                // CreateCommand::new(stringify!(#name)).description(#description)
                CreateCommand::new(stringify!(#name))
                    #description
            }
            
            // #visibility fn run(_options: &[ResolvedOption]) -> String {
            #visibility fn run(#(#args),*) -> String {
                #(#body)*
            }
        }
    };

    TokenStream::from(expanded)

}
