use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_quote, Attribute, Field, Meta, MetaList, PathArguments, Type};

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
        Self {
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

        if self.tenant_based {
            self.tuple = true;
        }
        if self.tuple {
            self.audit = true;
        }
        if self.audit || self.opt_lock || self.last_visit || self.user_based {
            self.storable = true;
        }
        if self.storable {
            self.base_data_model = true;
        }
    }

    #[allow(dead_code)]
    pub fn suitable_for_enum(&self) -> bool {
        !self.audit
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

    fn bdm_fields_of_builder_init() -> proc_macro2::TokenStream {
        quote! {}
    }

    fn bdm_fields_of_builder_fn() -> proc_macro2::TokenStream {
        quote! {}
    }

    fn storable_fields() -> proc_macro2::TokenStream {
        quote! {}
    }

    fn storable_fields_of_builder_fn() -> proc_macro2::TokenStream {
        quote! {}
    }

    fn storable_fields_of_builder_init() -> proc_macro2::TokenStream {
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

    fn audit_fields_of_builder_init() -> proc_macro2::TokenStream {
        quote! {
            created_at: None,
            created_by: None,
            last_modified_at: None,
            last_modified_by: None,
        }
    }

    fn audit_fields_of_builder_fn() -> proc_macro2::TokenStream {
        quote! {
            pub fn created_at(mut self, created_at: chrono::NaiveDateTime) -> Self {
                self.created_at = Some(created_at);
                self
            }

            pub fn created_by(mut self, created_by: UserId) -> Self {
                self.created_by = Some(created_by);
                self
            }

            pub fn last_modified_at(mut self, last_modified_at: chrono::NaiveDateTime) -> Self {
                self.last_modified_at = Some(last_modified_at);
                self
            }

            pub fn last_modified_by(mut self, last_modified_by: UserId) -> Self {
                self.last_modified_by = Some(last_modified_by);
                self
            }
        }
    }

    fn opt_lock_fields() -> proc_macro2::TokenStream {
        quote! {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub version: Option<u32>,
        }
    }

    fn opt_lock_fields_of_builder_init() -> proc_macro2::TokenStream {
        quote! {
            version: None,
        }
    }

    fn opt_lock_fields_of_builder_fn() -> proc_macro2::TokenStream {
        quote! {
            pub fn version(mut self, version: u32) -> Self {
                self.version = Some(version);
                self
            }
        }
    }

    fn last_visit_fields() -> proc_macro2::TokenStream {
        quote! {
            #[serde(skip_serializing_if = "Option::is_none", with = "option_naive_datetime")]
            pub last_visit_time: Option<chrono::NaiveDateTime>,
        }
    }

    fn last_visit_fields_of_builder_init() -> proc_macro2::TokenStream {
        quote! {
            last_visit_time: None,
        }
    }

    fn last_visit_fields_of_builder_fn() -> proc_macro2::TokenStream {
        quote! {
            pub fn last_visit_time(mut self, last_visit_time: chrono::NaiveDateTime) -> Self {
                self.last_visit_time = Some(last_visit_time);
                self
            }
        }
    }

    fn tuple_fields() -> proc_macro2::TokenStream {
        quote! {}
    }

    fn tuple_fields_of_builder_init() -> proc_macro2::TokenStream {
        quote! {}
    }

    fn tuple_fields_of_builder_fn() -> proc_macro2::TokenStream {
        quote! {}
    }

    fn tenant_based_fields() -> proc_macro2::TokenStream {
        quote! {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub tenant_id: Option<TenantId>,
        }
    }

    fn tenant_based_fields_of_builder_init() -> proc_macro2::TokenStream {
        quote! {
            tenant_id: None,
        }
    }

    fn tenant_based_fields_of_builder_fn() -> proc_macro2::TokenStream {
        quote! {
            pub fn tenant_id(mut self, tenant_id: TenantId) -> Self {
                self.tenant_id = Some(tenant_id);
                self
            }
        }
    }

    fn user_based_fields() -> proc_macro2::TokenStream {
        quote! {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub tenant_id: Option<TenantId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub user_id: Option<UserId>,
        }
    }

    fn user_based_fields_of_builder_init() -> proc_macro2::TokenStream {
        quote! {
            tenant_id: None,
            user_id: None,
        }
    }

    fn user_based_fields_of_builder_fn() -> proc_macro2::TokenStream {
        quote! {
            pub fn tenant_id(mut self, tenant_id: TenantId) -> Self {
                self.tenant_id = Some(tenant_id);
                self
            }

            pub fn user_id(mut self, user_id: UserId) -> Self {
                self.user_id = Some(user_id);
                self
            }
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

    pub fn fields_of_builder_init(&self) -> proc_macro2::TokenStream {
        let mut list = Vec::new();

        if self.base_data_model {
            list.push(Self::bdm_fields_of_builder_init());
        }
        if self.storable {
            list.push(Self::storable_fields_of_builder_init());
        }
        if self.tuple {
            list.push(Self::tuple_fields_of_builder_init());
        }
        if self.tenant_based {
            list.push(Self::tenant_based_fields_of_builder_init());
        }
        if self.user_based {
            list.push(Self::user_based_fields_of_builder_init());
        }
        if self.last_visit {
            list.push(Self::last_visit_fields_of_builder_init());
        }
        if self.opt_lock {
            list.push(Self::opt_lock_fields_of_builder_init());
        }
        if self.audit {
            list.push(Self::audit_fields_of_builder_init());
        }

        quote! {
            #(#list)*
        }
    }

    pub fn fields_of_builder_fn(&self) -> proc_macro2::TokenStream {
        let mut list = Vec::new();

        if self.base_data_model {
            list.push(Self::bdm_fields_of_builder_fn());
        }
        if self.storable {
            list.push(Self::storable_fields_of_builder_fn());
        }
        if self.tuple {
            list.push(Self::tuple_fields_of_builder_fn());
        }
        if self.tenant_based {
            list.push(Self::tenant_based_fields_of_builder_fn());
        }
        if self.user_based {
            list.push(Self::user_based_fields_of_builder_fn());
        }
        if self.last_visit {
            list.push(Self::last_visit_fields_of_builder_fn());
        }
        if self.opt_lock {
            list.push(Self::opt_lock_fields_of_builder_fn());
        }
        if self.audit {
            list.push(Self::audit_fields_of_builder_fn());
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
                fn tenant_id(&self) -> Option<TenantId> {
                    self.tenant_id.clone()
                }

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

    pub fn builder(
        &self,
        struct_name: &Ident,
        named_fields: &Punctuated<Field, Comma>,
    ) -> proc_macro2::TokenStream {
        let init_name_fields: Vec<proc_macro2::TokenStream> = named_fields
            .iter()
            .map(|f| {
                let field_name = &f.ident;
                if AdaptTo::is_option_field(f) {
                    quote! { #field_name: None,}
                } else {
                    let field_type = &f.ty.to_token_stream();
                    quote! { #field_name: #field_type::default(),}
                }
            })
            .collect();
        let init_new_fields = self.fields_of_builder_init();

        let fn_name_fields: Vec<proc_macro2::TokenStream> = named_fields
            .iter()
            .map(|f| {
                let field_name = &f.ident;
                let field_type = &f.ty.to_token_stream();
                if AdaptTo::is_option_field(f) {
                    let type_path = match &f.ty {
                        Type::Path(path) => path,
                        // checked in [is_option_field]
                        _ => unreachable!(),
                    };
                    let arguments = &type_path.path.segments.first().unwrap().arguments;
                    let field_type = match arguments {
                        PathArguments::AngleBracketed(args) => args.args.to_token_stream(),
                        _ => panic!("Field type of option must be angle bracketed."),
                    };
                    quote! {
                        pub fn #field_name(mut self, #field_name: #field_type) -> Self {
                            self.#field_name = Some(#field_name);
                            self
                        }
                    }
                } else {
                    quote! {
                        pub fn #field_name(mut self, #field_name: #field_type) -> Self {
                            self.#field_name = #field_name;
                            self
                        }
                    }
                }
            })
            .collect();
        let fn_new_fields = self.fields_of_builder_fn();

        quote! {
            impl #struct_name {
                pub fn new() -> Self {
                    Self {
                        #(#init_name_fields)*
                        #init_new_fields
                    }
                }

                #(#fn_name_fields)*
                #fn_new_fields

                pub fn build(self) -> Self {
                    self
                }
            }
        }
    }

    fn is_option_field(field: &Field) -> bool {
        if let Type::Path(type_path) = &field.ty {
            if let Some(segment) = type_path.path.segments.first() {
                if segment.ident == "Option" {
                    return true;
                }
            }
        }
        false
    }

    fn rebuild_existing_field_attributes(field: &Field) -> Vec<Attribute> {
        if !AdaptTo::is_option_field(field) {
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
