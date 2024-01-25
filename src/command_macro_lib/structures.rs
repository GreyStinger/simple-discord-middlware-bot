use quote::{ quote, ToTokens };
use proc_macro2::TokenStream as TokenStream2;
use syn::{ braced, parse::Parse, Attribute, Block, FnArg, Ident, Path, ReturnType, Stmt, Token, UseTree, Visibility };

use crate::util::Parenthesised;

#[derive(Debug)]
pub struct CommandFun {
    pub attrs: Vec<Attribute>,
    pub imports: Vec<UseTree>,
    pub visibility: Visibility,
    pub name: Ident,
    pub body: Vec<Stmt>,
}

impl Parse for CommandFun {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        let mut imports = Vec::new();
        while input.peek(Token![use]) {
            input.parse::<Token![use]>()?;
            imports.push(input.parse()?);
            input.parse::<Token![;]>()?;
        }

        let visibility = input.parse::<Visibility>()?;

        match input.parse::<Token![async]>() {
            Ok(_) => {
                println!("found async token");
            },
            Err(_) => {
                println!("no async token");
            },
        };

        input.parse::<Token![fn]>()?;

        let name = input.parse()?;

        let Parenthesised(_args) = input.parse::<Parenthesised<FnArg>>()?;

        let _ret = match input.parse::<ReturnType>()? {
            ReturnType::Type(_, t) => (*t).clone(),
            ReturnType::Default => {
                return Err(input
                    .error("expected a result type of either `CommandResult` or `CheckResult`"))
            },
        };

        let bcont;
        braced!(bcont in input);
        let body = bcont.call(Block::parse_within)?;

        Ok(CommandFun {
            attrs,
            imports,
            visibility,
            name,
            body,
        })
    }
}

impl ToTokens for CommandFun {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let Self { attrs: _, imports, visibility, name, body } = self;

        stream.extend(
            quote! {
            #visibility mod #name {
                use serenity::{builder::CreateCommand, all::ResolvedOption};
                #(#imports)*
                #visibility async fn register() -> CreateCommand {
                    CreateCommand::new(stringify!(#name)).description("It pings")
                }

                #visibility fn run(_options: &[ResolvedOption]) -> String {
                    #(#body)*
                }
            }
        }
        )
    }
}

#[allow(unused)]
pub fn is_rustfmt_or_clippy_attr(path: &Path) -> bool {
    path.segments.first().map_or(false, |s| (s.ident == "rustfmt" || s.ident == "clippy"))
}
