use std::fmt::{ self };

use proc_macro2::Span;
use syn::parse::{ Error, Parse, ParseStream, Parser, Result as SynResult };
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{ parenthesized, Attribute, Expr, Ident, Lit, LitStr, Meta, Path, Token };

/// `ValueKind` is an enumeration of the different kinds of values that can be associated with a name.
/// Each variant represents a different way that a value can be associated with a name.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(unused)]
pub enum ValueKind {
    /// The `Name` variant represents a name with no associated value.
    /// It corresponds to the `#[<name>]` syntax.
    Name,

    /// The `Equals` variant represents a name with a single associated value.
    /// It corresponds to the `#[<name> = <value>]` syntax.
    Equals,

    /// The `List` variant represents a name with multiple associated values.
    /// It corresponds to the `#[<name>([<value>, <value>, <value>, ...])]` syntax.
    List,

    /// The `SingleList` variant represents a name with a single associated value, but formatted as a list.
    /// It corresponds to the `#[<name>(<value>)]` syntax.
    SingleList,
}

/// `fmt::Display` is implemented for `ValueKind` to provide a human-readable representation of the enum.
/// This is used when formatting the enum as a string.
impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // For `Name`, it will display as "`#[<name>]`".
            Self::Name => f.pad("`#[<name>]`"),

            // For `Equals`, it will display as "`#[<name> = <value>]`".
            Self::Equals => f.pad("`#[<name> = <value>]`"),

            // For `List`, it will display as "`#[<name>([<value>, <value>, <value>, ...])]`".
            Self::List => f.pad("`#[<name>([<value>, <value>, <value>, ...])]`"),

            // For `SingleList`, it will display as "`#[<name>(<value>)]`".
            Self::SingleList => f.pad("`#[<name>(<value>)]`"),
        }
    }
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

/// `Values` is a public structure that holds the values associated with a name.
/// It has four public fields: `name`, `literals`, `kind`, and `span`.
#[derive(Debug)]
pub struct Values {
    /// `name` is an `Ident` that represents the name associated with the values.
    pub name: Ident,

    /// `literals` is a vector of `Lit` that holds the literal values associated with the name.
    pub literals: Vec<Lit>,

    /// `kind` is a `ValueKind` that represents the kind of values associated with the name.
    pub kind: ValueKind,

    /// `span` is a `Span` that represents the span of the source code where the values are defined.
    pub span: Span,
}

/// Implementation of `Values`.
impl Values {
    /// `new` is a public function that creates a new instance of `Values`.
    ///
    /// # Arguments
    ///
    /// * `name` - An `Ident` that represents the name associated with the values.
    /// * `kind` - A `ValueKind` that represents the kind of values associated with the name.
    /// * `literals` - A vector of `Lit` that holds the literal values associated with the name.
    /// * `span` - A `Span` that represents the span of the source code where the values are defined.
    ///
    /// # Returns
    ///
    /// A new instance of `Values`.
    #[inline]
    #[allow(unused)]
    pub fn new(name: Ident, kind: ValueKind, literals: Vec<Lit>, span: Span) -> Self {
        Values {
            name,
            literals,
            kind,
            span,
        }
    }
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
#[allow(unused)]
fn parse_values(attr: &Attribute) -> SynResult<Values> {
    let meta = attr.parse_args::<Meta>()?;

    match meta {
        Meta::Path(path) => {
            let name = path
                .get_ident()
                .ok_or_else(|| Error::new(path.span(), "expected identifier"))?
                .clone();

            Ok(Values::new(name, ValueKind::Name, Vec::new(), attr.span()))
        }
        Meta::List(meta) => {
            let name = meta.path
                .get_ident()
                .ok_or_else(|| Error::new(meta.path.span(), "expected identifier"))?
                .clone();
            let nested: SynResult<Punctuated<Expr, Token![,]>> = Parser::parse2(
                Punctuated::parse_terminated,
                meta.tokens
            );

            // syn v2 has made this so much more tricky :/
            // ! fixme: if this doesn't work, it's probably here Jay ;-;
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
                                Error::new(expr_path.path.span(), "expected identifier")
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
                .ok_or_else(|| Error::new(meta.path.span(), "expected identifier"))?
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
