pub use solana_utils_macro::VariantName;

/// This trait provides enums with a method to return the name the current variant.
///
/// # Example
///
/// ```
/// use solana_utils::VariantName;
///
/// #[derive(VariantName)]
/// enum Enum {
///     Foo,
///     Bar(),
///     Baz {},
/// }
///
/// assert_eq!(Enum::Foo.variant_name(), "Foo");
/// assert_eq!(Enum::Bar().variant_name(), "Bar");
/// assert_eq!(Enum::Baz {}.variant_name(), "Baz");
/// ```
pub trait VariantName {
    /// Returns the name of the enum variant.
    fn variant_name(&self) -> &'static str;
}
