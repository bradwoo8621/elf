mod enum_adapt;
mod model_adapt;
mod utils;

use crate::enum_adapt::{impl_display, impl_display_with_ampersand_prefix};
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
/// | user_based    | UserBasedTuple        | user_id                                           |
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
/// > for enum, only `bdm` is supported.
///
/// > All traits and return types are from watchmen_model module:
/// [BaseDataModel], [Storage], [Auditable], [OptimisticLock], [LastVisit],
/// [Tuple], [TenantBasedTuple], [UserBasedTuple], [UserId]
#[proc_macro_attribute]
pub fn adapt_model(attr: TokenStream, item: TokenStream) -> TokenStream {
    model_adapt::model_adapt(attr, item)
}

/// implement std::fmt::Display for enum.
/// enum variant name camel case to display with hyphen separated lowercase
/// use [[display = "name"]] to custom the display string.
#[proc_macro_derive(Display, attributes(display))]
pub fn impl_display_for_enum(item: TokenStream) -> TokenStream {
    impl_display(item)
}

/// implement std::fmt::Display for enum.
/// enum variant name camel case to display with first char lowercase and [&] as prefix.
/// use [[display = "name"]] to custom the display string.
///
/// # Examples
/// ```
/// pub enum Foo {
///    HelloWorld,
///    #[display = "&new"]
///    NewWorld
/// }
/// // expands to
/// impl std::fmt::Display for Foo {
///    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///       match self {
///          Foo::HelloWorld => f.write_fmt(::core::format_args!("{}", "&helloWorld")),
///          Foo::NewWorld => f.write_fmt(::core::format_args!("{}", "&new")),
///       }
///    }
/// }
/// ```
#[proc_macro_derive(DisplayWithAmpersandPrefix, attributes(display))]
pub fn impl_display_with_and_for_enum(item: TokenStream) -> TokenStream {
    impl_display_with_ampersand_prefix(item)
}
