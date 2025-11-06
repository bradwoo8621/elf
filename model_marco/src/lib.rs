use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Fields};

struct AdaptTo {
    base_data_model: bool,
    storable: bool,
    audit: bool,
    opt_lock: bool,
    last_visit: bool,
    tuple: bool,
    tenant_based: bool,
    user_based: bool,
}

impl AdaptTo {
    fn new() -> Self {
        AdaptTo {
            base_data_model: false,
            storable: false,
            audit: false,
            opt_lock: false,
            last_visit: false,
            tuple: false,
            tenant_based: false,
            user_based: false,
        }
    }

    fn set(&mut self, flag: &str) {
        match flag {
            "bdm" => self.base_data_model = true,
            "storable" => self.storable = true,
            "audit" => self.audit = true,
            "opt_lock" => self.opt_lock = true,
            "last_visit" => self.last_visit = true,
            "tuple" => self.tuple = true,
            "tenant_based" => self.tenant_based = true,
            "user_based" => self.user_based = true,
            _ => panic!("Unknown flag {}.", flag),
        }

        if self.user_based {
            self.tenant_based = true;
        }
        if self.tenant_based {
            self.tuple = true;
        }
        if self.tuple {
            self.audit = true;
        }
        if self.audit || self.opt_lock || self.last_visit {
            self.storable = true;
        }
        if self.storable {
            self.base_data_model = true;
        }
    }

    fn suitable_for_enum(&self) -> bool {
        !self.storable
            && !self.audit
            && !self.opt_lock
            && !self.last_visit
            && !self.tuple
            && !self.tenant_based
            && !self.user_based
    }

    fn serde_attribute(&self) -> proc_macro2::TokenStream {
        quote! {
            #[derive(serde::Serialize, serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
        }
    }

    fn attributes(&self) -> proc_macro2::TokenStream {
        let mut list = Vec::new();

        if self.storable {
            list.push(self.serde_attribute());
        }

        quote! {
            #(#list)*
        }
    }

    fn bdm_fields() -> proc_macro2::TokenStream {
        quote! {}
    }

    fn storable_fields() -> proc_macro2::TokenStream {
        quote! {}
    }

    fn audit_fields() -> proc_macro2::TokenStream {
        quote! {
            #[serde(skip_serializing_if = "Option::is_none", with = "option_naive_datetime")]
            pub created_at: Option<chrono::NaiveDateTime>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub created_by: Option<UserId>,
            #[serde(skip_serializing_if = "Option::is_none", with = "option_naive_datetime")]
            pub last_modified_at: Option<chrono::NaiveDateTime>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub last_modified_by: Option<UserId>,
        }
    }

    fn opt_lock_fields() -> proc_macro2::TokenStream {
        quote! {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub version: Option<u32>,
        }
    }

    fn last_visit_fields() -> proc_macro2::TokenStream {
        quote! {
            #[serde(skip_serializing_if = "Option::is_none", with = "option_naive_datetime")]
            pub last_visit_time: Option<chrono::NaiveDateTime>,
        }
    }

    fn tuple_fields() -> proc_macro2::TokenStream {
        quote! {}
    }

    fn tenant_based_fields() -> proc_macro2::TokenStream {
        quote! {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub tenant_id: Option<TenantId>,
        }
    }

    fn user_based_fields() -> proc_macro2::TokenStream {
        quote! {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub user_id: Option<UserId>,
        }
    }

    fn fields(&self) -> proc_macro2::TokenStream {
        let mut list = Vec::new();

        if self.base_data_model {
            list.push(Self::bdm_fields());
        }
        if self.storable {
            list.push(Self::storable_fields());
        }
        if self.tenant_based {
            list.push(Self::tenant_based_fields());
        }
        if self.user_based {
            list.push(Self::user_based_fields());
        }
        if self.tuple {
            list.push(Self::tuple_fields());
        }
        if self.opt_lock {
            list.push(Self::opt_lock_fields());
        }
        if self.last_visit {
            list.push(Self::last_visit_fields());
        }
        if self.audit {
            list.push(Self::audit_fields());
        }

        quote! {
            #(#list)*
        }
    }

    fn bdm_trait(struct_name: &Ident) -> proc_macro2::TokenStream {
        quote! {
            impl BaseDataModel for #struct_name {}
        }
    }

    fn storable_trait(struct_name: &Ident) -> proc_macro2::TokenStream {
        quote! {
            impl Storable for #struct_name {}
        }
    }

    fn audit_trait(struct_name: &Ident) -> proc_macro2::TokenStream {
        quote! {
            impl Auditable for #struct_name {
                fn created_at(&self) -> Option<chrono::NaiveDateTime> {
                    self.created_at
                }

                fn created_by(&self) -> Option<UserId> {
                    self.created_by.clone()
                }

                fn last_modified_at(&self) -> Option<chrono::NaiveDateTime> {
                    self.last_modified_at
                }

                fn last_modified_by(&self) -> Option<UserId> {
                    self.last_modified_by.clone()
                }
            }
        }
    }

    fn opt_lock_trait(struct_name: &Ident) -> proc_macro2::TokenStream {
        quote! {
            impl OptimisticLock for #struct_name {
                fn version(&self) -> Option<u32> {
                    self.version
                }
            }
        }
    }

    fn last_visit_trait(struct_name: &Ident) -> proc_macro2::TokenStream {
        quote! {
            impl LastVisit for #struct_name {
                fn last_visit_time(&self) -> Option<chrono::NaiveDateTime> {
                    self.last_visit_time
                }
            }
        }
    }

    fn tuple_trait(struct_name: &Ident) -> proc_macro2::TokenStream {
        quote! {
            impl Tuple for #struct_name {}
        }
    }

    fn tenant_based_trait(struct_name: &Ident) -> proc_macro2::TokenStream {
        quote! {
            impl TenantBasedTuple for #struct_name {
                fn tenant_id(&self) -> Option<TenantId> {
                    self.tenant_id.clone()
                }
            }
        }
    }

    fn user_based_trait(struct_name: &Ident) -> proc_macro2::TokenStream {
        quote! {
            impl UserBasedTuple for #struct_name {
                fn user_id(&self) -> Option<UserId> {
                    self.user_id.clone()
                }
            }
        }
    }

    fn traits(&self, struct_name: &Ident) -> proc_macro2::TokenStream {
        let mut list = Vec::new();

        if self.base_data_model {
            list.push(Self::bdm_trait(struct_name));
        }
        if self.storable {
            list.push(Self::storable_trait(struct_name));
        }
        if self.tenant_based {
            list.push(Self::tenant_based_trait(struct_name));
        }
        if self.user_based {
            list.push(Self::user_based_trait(struct_name));
        }
        if self.tuple {
            list.push(Self::tuple_trait(struct_name));
        }
        if self.opt_lock {
            list.push(Self::opt_lock_trait(struct_name));
        }
        if self.last_visit {
            list.push(Self::last_visit_trait(struct_name));
        }
        if self.audit {
            list.push(Self::audit_trait(struct_name));
        }

        quote! {
            #(#list)*
        }
    }
}

/// Adapt the model struct or enum to various traits and fields based on the provided attributes.
/// For struct, supported trait are:
/// - bdm: BaseDataModel
/// - storable: Storable
/// - audit: Auditable
/// - opt_lock: OptimisticLock
/// - last_visit: LastVisit
/// - tuple: Tuple
/// - tenant_based: TenantBasedTuple
/// - user_based: UserBasedTuple
///
/// Supported attributes are:
/// - bdm
/// - storable
/// - audit
///     - created_at
///     - created_by
///     - last_modified_at
///     - last_modified_by
/// - opt_lock
///     - version
/// - last_visit
///     - last_visit_time
/// - tuple
/// - tenant_based
///     - tenant_id
/// - user_based
///    - user_id
///
/// # Examples
/// #[adapt_model(opt_lock, tenant_based)]
/// pub struct User {
///    pub user_id: Option<UserId>,
/// }
/// expands to
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
///
/// for enum, only bdm is supported.
#[proc_macro_attribute]
pub fn adapt_model(attr: TokenStream, item: TokenStream) -> TokenStream {
    if attr.is_empty() {
        panic!(
            "This macro only works on attributes [bdm, storable, audit, opt_lock, tuple, tenant_based, user_based, last_visit]."
        )
    }

    let attr_str = attr.to_string();
    let meta: Vec<&str> = attr_str.split(",").map(|s| s.trim()).collect();
    let mut adapt_to = AdaptTo::new();
    for (_, target) in meta.iter().enumerate() {
        adapt_to.set(*target);
    }

    let input = parse_macro_input!(item as DeriveInput);
    let vis = &input.vis.to_token_stream();
    let input_name = &input.ident;
    match input.data {
        syn::Data::Struct(s) => match s.fields {
            Fields::Named(named_fields) => {
                let named_field_list = named_fields.named;
                let existing_fields = named_field_list.to_token_stream();
                // modifications
                let attributes = adapt_to.attributes();
                let new_fields = adapt_to.fields();
                let traits = adapt_to.traits(input_name);
                let expanded = quote! {
                    #attributes
                    #vis struct #input_name {
                        #existing_fields
                        #new_fields
                    }

                    #traits
                };
                TokenStream::from(expanded)
            }
            Fields::Unnamed(_) => {
                panic!("This macro only works on structs with name field.")
            }
            Fields::Unit => {
                panic!("This macro only works on structs with name field.")
            }
        },
        syn::Data::Enum(e) => {
            if !adapt_to.suitable_for_enum() {
                panic!("Enums can only adapt to [bdm].");
            }
            let variants = e.variants.to_token_stream();
            // modifications
            let attributes = adapt_to.attributes();
            let traits = adapt_to.traits(input_name);
            let expanded = quote! {
                #attributes
                #vis enum #input_name {
                    #variants
                }

                #traits
            };
            TokenStream::from(expanded)
        }
        _ => panic!("This macro only works on structs."),
    }
}
