use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_quote, Attribute, Field, Meta, MetaList, Type};

pub struct AdaptTo {
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
    pub fn new() -> Self {
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

    pub fn set(&mut self, flag: &str) {
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

    pub fn suitable_for_enum(&self) -> bool {
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
            #[serde(deny_unknown_fields, rename_all = "camelCase")]
        }
    }

    pub fn attributes(&self) -> proc_macro2::TokenStream {
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

    pub fn fields(&self) -> proc_macro2::TokenStream {
        let mut list = Vec::new();

        if self.base_data_model {
            list.push(Self::bdm_fields());
        }
        if self.storable {
            list.push(Self::storable_fields());
        }
        if self.tuple {
            list.push(Self::tuple_fields());
        }
        if self.tenant_based {
            list.push(Self::tenant_based_fields());
        }
        if self.user_based {
            list.push(Self::user_based_fields());
        }
        if self.last_visit {
            list.push(Self::last_visit_fields());
        }
        if self.opt_lock {
            list.push(Self::opt_lock_fields());
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

    pub fn traits(&self, struct_name: &Ident) -> proc_macro2::TokenStream {
        let mut list = Vec::new();

        if self.base_data_model {
            list.push(Self::bdm_trait(struct_name));
        }
        if self.storable {
            list.push(Self::storable_trait(struct_name));
        }
        if self.tuple {
            list.push(Self::tuple_trait(struct_name));
        }
        if self.tenant_based {
            list.push(Self::tenant_based_trait(struct_name));
        }
        if self.user_based {
            list.push(Self::user_based_trait(struct_name));
        }
        if self.last_visit {
            list.push(Self::last_visit_trait(struct_name));
        }
        if self.opt_lock {
            list.push(Self::opt_lock_trait(struct_name));
        }
        if self.audit {
            list.push(Self::audit_trait(struct_name));
        }

        quote! {
            #(#list)*
        }
    }

    fn rebuild_existing_field_attributes(field: &Field) -> Vec<Attribute> {
        let mut is_option = false;

        if let Type::Path(type_path) = &field.ty {
            if let Some(segment) = type_path.path.segments.first() {
                if segment.ident == "Option" {
                    is_option = true;
                }
            }
        }

        if !is_option {
            return field.attrs.clone();
        }

        let mut attrs = Vec::new();
        let mut modified = false;
        for attr in &field.attrs {
            // println!("{}", attr.meta.path().is_ident("serde"));
            if !attr.meta.path().is_ident("serde") {
                attrs.push(attr.clone());
                continue;
            }

            modified = true;

            match &attr.meta {
                Meta::List(list) => {
                    if list.tokens.to_string().contains("skip_serializing_if") {
                        // already defined, do nothing
                        attrs.push(attr.clone());
                    } else {
                        let tokens = list.tokens.to_token_stream();

                        attrs.push(Attribute {
                            pound_token: attr.pound_token.clone(),
                            style: attr.style.clone(),
                            bracket_token: attr.bracket_token.clone(),
                            meta: Meta::List(MetaList {
                                path: list.path.clone(),
                                delimiter: list.delimiter.clone(),
                                tokens: quote! { #tokens, skip_serializing_if = "Option::is_none" },
                            }),
                        })
                    }
                }
                _ => {
                    // do nothing, this attr serde is not in correct syntax
                    attrs.push(attr.clone());
                }
            }
        }

        if !modified {
            attrs.push(parse_quote! { #[serde(skip_serializing_if = "Option::is_none")] });
        }
        // println!("{}", attrs.iter().map(|a| a.to_token_stream().to_string()).collect::<String>());
        attrs
    }

    fn rebuild_existing_field(field: &Field) -> Field {
        Field {
            attrs: Self::rebuild_existing_field_attributes(field),
            vis: field.vis.clone(),
            mutability: field.mutability.clone(),
            ident: field.ident.clone(),
            colon_token: field.colon_token.clone(),
            ty: field.ty.clone(),
        }
    }

    pub fn rebuild_existing_fields(
        &self,
        named_fields: &Punctuated<Field, Comma>,
    ) -> proc_macro2::TokenStream {
        if !self.storable {
            return named_fields.to_token_stream();
        }

        let modified_fields: Vec<proc_macro2::TokenStream> = named_fields
            .iter()
            .map(Self::rebuild_existing_field)
            .map(|f| quote! { #f,})
            .collect();
        quote! {
            #(#modified_fields)*
        }
    }
}
