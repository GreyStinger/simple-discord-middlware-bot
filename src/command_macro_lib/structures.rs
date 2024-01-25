use quote::{ quote, ToTokens };
use proc_macro2::TokenStream as TokenStream2;
use syn::{
    braced,
    parse::{ Parse, Error },
    spanned::Spanned,
    Attribute,
    Block,
    FnArg,
    Ident,
    Pat,
    Path,
    Result,
    ReturnType,
    Stmt,
    Token,
    // UseTree,
    Visibility,
};

use crate::{ util::Parenthesised, Argument };

fn parse_argument(arg: FnArg) -> Result<Argument> {
    match arg {
        FnArg::Typed(typed) => {
            let pat = typed.pat;
            let kind = typed.ty;

            match *pat {
                Pat::Ident(id) => {
                    let name = id.ident;
                    let mutable = id.mutability;

                    Ok(Argument {
                        mutable,
                        name,
                        kind: *kind,
                    })
                }
                Pat::Wild(wild) => {
                    let token = wild.underscore_token;

                    let name = Ident::new("_", token.spans[0]);

                    Ok(Argument {
                        mutable: None,
                        name,
                        kind: *kind,
                    })
                }
                _ => Err(Error::new(pat.span(), format_args!("unsupported pattern: {pat:?}"))),
            }
        }
        FnArg::Receiver(_) => {
            Err(Error::new(arg.span(), format_args!("`self` arguments are prohibited: {arg:?}")))
        }
    }
}

#[derive(Debug)]
pub struct CommandFun {
    pub attrs: Vec<Attribute>,
    // pub imports: Vec<UseTree>,
    pub visibility: Visibility,
    pub name: Ident,
    pub args: Vec<Argument>,
    pub body: Vec<Stmt>,
}

impl Parse for CommandFun {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        // ?fixme: Have to look into this, seems like macro used on import only gives import stream to proc_macro 
        // let mut imports = Vec::new();
        // while input.peek(Token![use]) {
        //     input.parse::<Token![use]>()?;
        //     imports.push(input.parse()?);
        //     input.parse::<Token![;]>()?;
        // }

        let visibility = input.parse::<Visibility>()?;

        // ?: Do I need this?
        // match input.parse::<Token![async]>() {
        //     Ok(_) => {
        //         // println!("found async token");
        //     }
        //     Err(_) => {
        //         // println!("no async token");
        //     }
        // }

        input.parse::<Token![fn]>()?;

        let name = input.parse()?;

        let Parenthesised(args) = input.parse::<Parenthesised<FnArg>>()?;

        let _ret = match input.parse::<ReturnType>()? {
            ReturnType::Type(_, t) => (*t).clone(),
            ReturnType::Default => {
                return Err(
                    input.error("expected a result type of either `CommandResult` or `CheckResult`")
                );
            }
        };

        let bcont;
        braced!(bcont in input);
        let body = bcont.call(Block::parse_within)?;

        let args = args.into_iter().map(parse_argument).collect::<Result<Vec<_>>>()?;

        Ok(CommandFun {
            attrs,
            // imports,
            visibility,
            name,
            args,
            body,
        })
    }
}

impl ToTokens for CommandFun {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let Self { attrs: _, 
            // imports, 
            visibility, name, args, body } = self;

        stream.extend(
            quote! {
            // #(#imports)*
            #visibility fn #name(#(#args),*) {
                #(#body)*
            }
        }
        )
    }
}

#[allow(unused)]
pub fn is_rustfmt_or_clippy_attr(path: &Path) -> bool {
    path.segments.first().map_or(false, |s| (s.ident == "rustfmt" || s.ident == "clippy"))
}
