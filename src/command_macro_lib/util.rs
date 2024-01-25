use proc_macro::TokenStream;
use syn::parse::{ Error, Parse, ParseStream, Parser, Result as SynResult };
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{ parenthesized, Attribute, Expr, Ident, Lit, LitStr, Meta, Path, Token };

use crate::attributes::{ValueKind, Values};

#[inline]
pub fn into_stream(e: &Error) -> TokenStream {
    e.to_compile_error().into()
}

macro_rules! propagate_err {
    ($res:expr) => {{
        match $res {
            Ok(v) => v,
            Err(e) => return $crate::util::into_stream(&e),
        }
    }};
}

/// Converts a `Path` to an `Ident`.
///
/// This function takes a reference to a `Path` as an argument and attempts to convert it to an `Ident`.
/// It returns a `Result` which is `Ok` if the conversion is successful, and `Err` otherwise.
///
/// # Arguments
///
/// * `p` - A reference to a `Path` that is to be converted to an `Ident`.
///
/// # Errors
///
/// This function will return an `Error` in the following situations:
///
/// * If the `Path` is empty (i.e., it has no segments), the function cannot convert it to an identifier and will return an error.
/// * If the `Path` has more than one segment, the function will return an error as it expects the path to have only one segment.
/// * If the singular path segment has any arguments, the function will return an error as it expects the path segment to not have any arguments.
///
/// # Returns
///
/// * An `Ok` variant containing the `Ident` if the conversion is successful.
/// * An `Err` variant containing an `Error` if the conversion is unsuccessful.
///
/// # Example
///
/// ```rust
/// let path = syn::parse_str::<syn::Path>("my_ident").unwrap();
/// let ident = to_ident(&path).unwrap();
/// assert_eq!(ident, syn::Ident::new("my_ident", proc_macro2::Span::call_site()));
/// ```
#[allow(unused)]
fn to_ident(p: &Path) -> SynResult<Ident> {
    if p.segments.is_empty() {
        return Err(Error::new(p.span(), "cannot convert an empty path to an identifier"));
    }

    if p.segments.len() > 1 {
        return Err(Error::new(p.span(), "the path must not have more than one segment"));
    }

    if !p.segments[0].arguments.is_empty() {
        return Err(Error::new(p.span(), "the singular path segment must not have any arguments"));
    }

    Ok(p.segments[0].ident.clone())
}

/// Parses the values from an attribute and returns a `Values` struct.
///
/// This function takes a reference to an `Attribute` as an argument and attempts to parse it into a `Values` struct.
/// It returns a `Result` which is `Ok` if the parsing is successful, and `Err` otherwise.
///
/// # Arguments
///
/// * `attr` - A reference to an `Attribute` that is to be parsed into a `Values` struct.
///
/// # Errors
///
/// This function will return an `Error` in the following situations:
///
/// * If the `Path` is empty (i.e., it has no segments), the function cannot convert it to an identifier and will return an error.
/// * If the `Path` has more than one segment, the function will return an error as it expects the path to have only one segment.
/// * If the singular path segment has any arguments, the function will return an error as it expects the path segment to not have any arguments.
/// * If the list is empty, the function will return an error.
/// * If the expression is not a literal or an identifier, the function will return an error.
/// * If the value is not a literal, the function will return an error.
///
/// # Returns
///
/// * An `Ok` variant containing the `Values` struct if the parsing is successful.
/// * An `Err` variant containing an `Error` if the parsing is unsuccessful.
///
/// # Example
///
/// ```rust
/// let attr = syn::parse_str::<syn::Attribute>("#[my_attr = \"my_value\"]").unwrap();
/// let values = parse_values(&attr).unwrap();
/// assert_eq!(values.name, syn::Ident::new("my_attr", proc_macro2::Span::call_site()));
/// assert_eq!(values.kind, ValueKind::Equals);
/// assert_eq!(values.literals.len(), 1);
/// ```
pub fn parse_values(attr: &Attribute) -> SynResult<Values> {
    println!("parsing meta");
    // let meta = attr.parse_args::<Meta>()?;
    // attr.meta

    // println!("proceeding with match statement for parse_values");
    // dbg!(meta.clone());
    match &attr.meta {
        Meta::Path(path) => {
            let name = path
                .get_ident()
                .ok_or_else(|| Error::new(path.span(), "expected identifier (path)"))?
                .clone();

            Ok(Values::new(name, ValueKind::Name, Vec::new(), attr.span()))
        }
        Meta::List(meta) => {
            let name = meta.path
                .get_ident()
                .ok_or_else(|| Error::new(meta.path.span(), "expected identifier (list)"))?
                .clone();
            let nested: SynResult<Punctuated<Expr, Token![,]>> = Parser::parse2(
                Punctuated::parse_terminated,
                meta.tokens.clone()
            );

            let nested = match nested {
                Ok(nested) => nested,
                Err(_) => {
                    return Err(Error::new(attr.span(), "list cannot be empty"));
                }
            };

            let mut lits = Vec::with_capacity(nested.len());

            for expr in nested {
                match expr {
                    Expr::Lit(expr_lit) => lits.push(expr_lit.lit.clone()),
                    Expr::Path(expr_path) => {
                        let i = expr_path.path
                            .get_ident()
                            .ok_or_else(||
                                Error::new(expr_path.path.span(), "expected identifier (nested)")
                            )?
                            .clone();
                        lits.push(Lit::Str(LitStr::new(&i.to_string(), i.span())));
                    }
                    _ => {
                        return Err(
                            Error::new(
                                expr.span(),
                                "cannot nest a list; only accept literals and identifiers at this level"
                            )
                        );
                    }
                }
            }

            let kind = if lits.len() == 1 { ValueKind::SingleList } else { ValueKind::List };

            Ok(Values::new(name, kind, lits, attr.span()))
        }
        Meta::NameValue(meta) => {
            let name = meta.path
                .get_ident()
                .ok_or_else(|| Error::new(meta.path.span(), "expected identifier (name_value)"))?
                .clone();
            let lit = match &meta.value {
                Expr::Lit(expr_lit) => expr_lit.lit.clone(),
                _ => {
                    return Err(Error::new(meta.value.span(), "expected literal"));
                }
            };

            Ok(Values::new(name, ValueKind::Equals, vec![lit], attr.span()))
        }
    }
}

#[derive(Debug)]
pub struct Parenthesised<T>(pub Punctuated<T, Comma>);

impl<T: Parse> Parse for Parenthesised<T> {
    fn parse(input: ParseStream<'_>) -> SynResult<Self> {
        let content;
        parenthesized!(content in input);

        Ok(Parenthesised(content.parse_terminated(T::parse, Comma)?))
    }
}
