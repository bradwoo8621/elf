mod enum_adapt;
mod model_adapt;
mod utils;

use crate::enum_adapt::{impl_display, impl_enum_type, impl_serde, impl_str_enum};
use proc_macro::TokenStream;

/// Adapt the model struct or enum to various traits and fields based on the provided attributes.
/// For struct:
///
/// | keyword       | trait                  | fields added                                      |
/// |---------------|------------------------|---------------------------------------------------|
/// | bdm           | BaseDataModel         | None                                              |
/// | storable      | Storable              | None                                              |
/// | audit         | Auditable             | created_at, created_by, last_modified_at, last_modified_by |
/// | opt_lock      | OptimisticLock        | version                                           |
/// | last_visit    | LastVisit             | last_visit_time                                   |
/// | tuple         | Tuple                 | None                                              |
/// | tenant_based  | TenantBasedTuple      | tenant_id                                         |
/// | user_based    | UserBasedTuple        | tenant_id, user_id                                |
///
/// # Examples
/// ```
/// #[adapt_model(opt_lock, tenant_based)]
/// pub struct User {
///    pub user_id: Option<UserId>,
/// }
/// // expands to
/// pub struct User {
///    pub user_id: Option<UserId>,
///    // adapted fields
///    pub tenant_id: Option<TenantId>,
///    pub version: Option<u32>,
///    pub created_at: Option<chrono::NaiveDateTime>,
///    pub created_by: Option<UserId>,
///    pub last_modified_at: Option<chrono::NaiveDateTime>,
///    pub last_modified_by: Option<UserId>,
/// }
/// ```
///
/// If `Storable` implemented, struct will automatically add follow:
/// ```rust
/// #[derive(serde::Serialize, serde::Deserialize)]
/// #[serde(deny_unknown_fields, rename_all = "camelCase")]
/// ```
/// and add following to fields which type has `Option<>`:
/// ```rust
/// #[serde(skip_serializing_if = "Option::is_none")]
/// ```
/// if `#[serde]` already defined on field, `skip_serializing_if = "Option::is_none"` will be added.
///
/// All traits and return types are from [watchmen_model] module:
/// So import them by yourself, such as:
/// ```
/// use watchmen_model::{
///     BaseDataModel, Storable,
///     Auditable, OptimisticLock, LastVisit,
///     Tuple, TenantBasedTuple, UserBasedTuple,
///     TenantId, UserId
/// };
/// ```
#[proc_macro_attribute]
pub fn adapt_model(attr: TokenStream, item: TokenStream) -> TokenStream {
    model_adapt::model_adapt(attr, item)
}

/// implement [std::fmt::Display] for enum.
/// enum variant name camel case to display with hyphen separated lowercase
/// - use [#[pattern = "type"]] on enum to custom the display string transformation rule,
///   Available pattern are:
///   - [kebab-lower]: Converts variant name from camel case (e.g., “CamelCase”) to kebab case (e.g., “camel-case”)
///     by inserting hyphens before each uppercase letter except the first one
///     and converting all letters to lowercase.
///     It is the default when not appointed.
///   - [kebab-upper]: Converts variant name from camel case (e.g., “CamelCase”) to kebab case (e.g., “camel-case”)
///     by inserting hyphens before each uppercase letter except the first one
///     and converting all letters to uppercase.
///   - [ampersand-prefix]: Prepends an ampersand to the input string
///     and converts the first character of the string to its ASCII lowercase form;
///     if the input string is empty, it simply returns a string consisting of only an ampersand.
///   - [keep-same]: Same as variant name.
///   - [upper-case]: Converts variant name to uppercase.
///   - [lower-case]: Converts variant name to lowercase.
/// - use [#[display = "name"]] on fields to custom the display string.
#[proc_macro_derive(Display, attributes(pattern, display))]
pub fn impl_display_for_enum(item: TokenStream) -> TokenStream {
    impl_display(item)
}

/// implement [serde::Serialize] and [serde::Deserialize] for enum.
/// variant name transform refer to [impl_display_for_enum]
#[proc_macro_derive(Serde, attributes(pattern, display))]
pub fn impl_serde_for_enum(item: TokenStream) -> TokenStream {
    impl_serde(item)
}

/// Prove that this enumeration defines a set of value types.
#[proc_macro_derive(VariousValueTypes)]
pub fn impl_various_value_types_form_enum(item: TokenStream) -> TokenStream {
    impl_enum_type(item)
}

/// Prove that this enumeration defines a set of struct types.
#[proc_macro_derive(VariousStructTypes)]
pub fn impl_various_struct_types_form_enum(item: TokenStream) -> TokenStream {
    impl_enum_type(item)
}

/// Prove that this enumeration defines a set of string constants.
/// and implements it with a parse function to transform string to enum
/// variant name transform refer to [impl_display_for_enum]
#[proc_macro_derive(StrEnum, attributes(pattern, display))]
pub fn impl_str_enum_form_enum(item: TokenStream) -> TokenStream {
    impl_str_enum(item)
}
