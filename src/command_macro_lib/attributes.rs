use std::fmt::{ self };

use proc_macro2::Span;
use syn::{Ident, Lit};

// #[inline]
// pub fn _parse<T: AttributeOption>(values: Values) -> Result<T> {
//     T::parse(values)
// }

// pub trait AttributeOption: Sized {
//     fn parse(values: Values) -> Result<Self>;
// }

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
    pub fn new(name: Ident, kind: ValueKind, literals: Vec<Lit>, span: Span) -> Self {
        Values {
            name,
            literals,
            kind,
            span,
        }
    }
}

