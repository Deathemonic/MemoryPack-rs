use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::visit::{self, Visit};
use syn::{
    Data,
    DeriveInput,
    Field,
    GenericParam,
    Generics,
    Ident,
    Lifetime,
    LifetimeParam,
    WherePredicate,
    parse_quote
};

use crate::generation::attributes::FieldAttributes;

#[derive(Clone, Copy)]
pub enum DeserializationMode {
    Input,
    Arena
}

pub struct OrderedField<'a> {
    pub order: usize,
    pub field: &'a Field,
    pub ident: &'a Option<syn::Ident>
}

struct TypeParameterUsage<'a> {
    parameter: &'a Ident,
    used: bool
}

impl DeserializationMode {
    pub fn deserialize_value(self) -> TokenStream {
        match self {
            Self::Input => quote!(memorypack::MemoryPackDeserialize::deserialize(reader)?),
            Self::Arena => {
                quote!(memorypack::MemoryPackDeserializeIn::deserialize_in(reader, arena)?)
            }
        }
    }

    pub fn trait_path(self, lifetime: &Lifetime, arena_type: &Ident) -> TokenStream {
        match self {
            Self::Input => quote!(memorypack::MemoryPackDeserialize<#lifetime>),
            Self::Arena => quote!(memorypack::MemoryPackDeserializeIn<#lifetime, #arena_type>)
        }
    }

    pub const fn lifetime_name(self) -> &'static str {
        match self {
            Self::Input => "'de",
            Self::Arena => "'arena"
        }
    }
}

impl<'ast> Visit<'ast> for TypeParameterUsage<'_> {
    fn visit_type_path(&mut self, path: &'ast syn::TypePath) {
        self.used |= path
            .path
            .segments
            .iter()
            .any(|segment| segment.ident == *self.parameter);

        visit::visit_type_path(self, path);
    }
}

#[inline]
pub fn is_single_field_i32(data_struct: &syn::DataStruct) -> bool {
    use syn::Fields;
    matches!(&data_struct.fields,
        Fields::Unnamed(fields) if fields.unnamed.len() == 1
            && matches!(&fields.unnamed[0].ty,
                syn::Type::Path(type_path) if type_path.path.is_ident("i32")
            )
    )
}

pub fn has_repr_c(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("repr") {
            return false;
        }

        attr.meta.require_list().is_ok_and(|list| {
            list.tokens
                .to_string()
                .split(',')
                .any(|token| token.trim() == "C")
        })
    })
}

#[inline]
pub fn has_explicit_discriminants(data_enum: &syn::DataEnum) -> bool {
    data_enum
        .variants
        .iter()
        .all(|v| v.discriminant.is_some())
}

#[inline]
pub fn should_skip_field(field: &Field) -> bool {
    FieldAttributes::for_field(field).skipped
        || field
            .ident
            .as_ref()
            .is_some_and(|ident| ident.to_string().starts_with('_'))
}

pub fn get_field_order(field: &Field) -> Option<usize> { FieldAttributes::for_field(field).order }

pub fn prepare_ordered_fields<'a>(fields: &'a [&'a Field]) -> Vec<OrderedField<'a>> {
    let mut ordered: Vec<_> = fields
        .iter()
        .enumerate()
        .map(|(index, f)| OrderedField {
            order: get_field_order(f).unwrap_or(index),
            field: f,
            ident: &f.ident
        })
        .collect();

    ordered.sort_by_key(|f| f.order);
    ordered
}

pub fn generate_field_deserialize(field: &Field, mode: DeserializationMode) -> TokenStream {
    let name = &field.ident;

    if should_skip_field(field) {
        return quote! { let #name = Default::default(); };
    }

    let deserialize_value = generate_field_deserialize_value(field, mode);

    quote! { let #name = #deserialize_value; }
}

pub fn generate_field_serialize(field: &Field, value: &TokenStream) -> TokenStream {
    FieldAttributes::for_field(field).formatter.map_or_else(
        || quote! { memorypack::MemoryPackSerialize::serialize(#value, writer)?; },
        |formatter| {
            let field_type = &field.ty;

            quote! { <#formatter as memorypack::MemoryPackFormatter<#field_type>>::serialize(#value, writer)?; }
        }
    )
}

pub fn generate_field_deserialize_value(field: &Field, mode: DeserializationMode) -> TokenStream {
    FieldAttributes::for_field(field).formatter.map_or_else(
        || mode.deserialize_value(),
        |formatter| {
            let field_type = &field.ty;

            quote! { <#formatter as memorypack::MemoryPackFormatter<#field_type>>::deserialize(reader)? }
        }
    )
}

pub fn generate_field_size_hint(field: &Field, value: &TokenStream) -> TokenStream {
    FieldAttributes::for_field(field).formatter.map_or_else(
        || quote! { memorypack::MemoryPackSerialize::serialized_size_hint(#value) },
        |formatter| {
            let field_type = &field.ty;

            quote! { <#formatter as memorypack::MemoryPackFormatter<#field_type>>::serialized_size_hint(#value) }
        }
    )
}

pub fn generate_size_hint(
    data: &syn::Data,
    is_union: bool,
    is_version_tolerant: bool,
    is_circular: bool
) -> proc_macro2::TokenStream {
    if is_circular {
        return quote! { 0 };
    }

    match data {
        syn::Data::Struct(data_struct) => {
            let header_size = 1_usize;

            match &data_struct.fields {
                syn::Fields::Named(fields) => {
                    let fields: Vec<_> = fields
                        .named
                        .iter()
                        .filter(|field| !should_skip_field(field))
                        .map(|field| {
                            let name = &field.ident;

                            generate_field_size_hint(field, &quote! { &self.#name })
                        })
                        .collect();
                    let length_table_size = if is_version_tolerant { fields.len() } else { 0 };

                    quote! {
                        #header_size + #length_table_size
                            #( + #fields )*
                    }
                }
                syn::Fields::Unnamed(fields) => {
                    let hints: Vec<_> = fields
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(index, field)| {
                            let index = syn::Index::from(index);

                            generate_field_size_hint(field, &quote! { &self.#index })
                        })
                        .collect();
                    let length_table_size = if is_version_tolerant { hints.len() } else { 0 };

                    quote! {
                        #header_size + #length_table_size
                            #( + #hints )*
                    }
                }
                syn::Fields::Unit => quote! { #header_size }
            }
        }
        syn::Data::Enum(data_enum) if is_union => {
            let variants = data_enum.variants.iter().map(|variant| {
                let variant_name = &variant.ident;

                quote! {
                    Self::#variant_name(inner) => {
                        1 + memorypack::MemoryPackSerialize::serialized_size_hint(inner)
                    }
                }
            });

            quote! { match self { #(#variants),* } }
        }
        syn::Data::Enum(_) => quote! { 4 },
        syn::Data::Union(_) => quote! { 0 }
    }
}

pub fn prepare_serialization_generics(input: &DeriveInput) -> Generics {
    let mut serialization_generics = input.generics.clone();

    serialization_generics
        .type_params_mut()
        .filter(|parameter| requires_default_traits(input, &parameter.ident))
        .for_each(|parameter| {
            parameter
                .bounds
                .push(parse_quote!(memorypack::MemoryPackSerialize));
        });

    add_formatter_bounds(&mut serialization_generics, input);
    add_unmanaged_bounds(&mut serialization_generics, input);

    serialization_generics
}

pub fn add_formatter_bounds(generics: &mut Generics, input: &DeriveInput) {
    for field in input_fields(input) {
        if let Some(formatter) = FieldAttributes::for_field(field).formatter {
            let field_type = &field.ty;

            generics
                .make_where_clause()
                .predicates
                .push(parse_quote!(
                    #formatter: memorypack::MemoryPackFormatter<#field_type>
                ));
        }
    }
}

pub fn add_unmanaged_bounds(generics: &mut Generics, input: &DeriveInput) {
    if !has_repr_c(&input.attrs) {
        return;
    }

    for field in input_fields(input) {
        let field_type = &field.ty;

        generics
            .make_where_clause()
            .predicates
            .push(parse_quote!(
                #field_type: memorypack::MemoryPackFixedSize
            ));
    }
}

pub fn generate_deserialization_impl(
    input: &DeriveInput,
    mode: DeserializationMode,
    body: &TokenStream,
    is_value_type: bool
) -> TokenStream {
    let name = &input.ident;
    let (_, type_generics, _) = input.generics.split_for_impl();
    let (generics, lifetime, arena_type) = prepare_deserialization_generics(input, mode);
    let (impl_generics, _, where_clause) = generics.split_for_impl();
    let trait_path = mode.trait_path(&lifetime, &arena_type);
    let method = generate_deserialization_method(mode, &lifetime, &arena_type, body);
    let nullable_method = is_value_type.then(|| quote! {
        fn deserialize_nullable(reader: &mut memorypack::MemoryPackReader<#lifetime>) -> Result<Option<Self>, memorypack::MemoryPackError> {
            <Self as memorypack::MemoryPackFixedSize>::deserialize_nullable_value(reader)
        }
    });

    quote! {
        impl #impl_generics #trait_path for #name #type_generics #where_clause {
            #[inline]
            #method

            #nullable_method
        }
    }
}

fn prepare_deserialization_generics(
    input: &DeriveInput,
    mode: DeserializationMode
) -> (Generics, Lifetime, Ident) {
    let input_lifetime = prepare_input_lifetime(&input.generics, mode.lifetime_name());
    let lifetime_parameter = LifetimeParam::new(input_lifetime.clone());
    let arena_type = prepare_arena_type(&input.generics);
    let trait_path = mode.trait_path(&input_lifetime, &arena_type);

    let mut deserialization_generics = input.generics.clone();
    deserialization_generics
        .params
        .insert(0, GenericParam::Lifetime(lifetime_parameter));

    for parameter in deserialization_generics.type_params_mut() {
        if requires_default_traits(input, &parameter.ident) {
            parameter.bounds.push(parse_quote!(#trait_path));
        }
    }

    if matches!(mode, DeserializationMode::Arena) {
        deserialization_generics.params.push(parse_quote!(
            #arena_type: memorypack::MemoryPackArena + ?Sized
        ));
    }

    add_field_deserialization_bounds(&mut deserialization_generics, input, &trait_path);
    add_formatter_bounds(&mut deserialization_generics, input);
    add_unmanaged_bounds(&mut deserialization_generics, input);

    (deserialization_generics, input_lifetime, arena_type)
}

fn generate_deserialization_method(
    mode: DeserializationMode,
    lifetime: &Lifetime,
    arena_type: &Ident,
    body: &TokenStream
) -> TokenStream {
    match mode {
        DeserializationMode::Input => quote! {
            fn deserialize(reader: &mut memorypack::MemoryPackReader<#lifetime>) -> Result<Self, memorypack::MemoryPackError> {
                #body
            }
        },
        DeserializationMode::Arena => quote! {
            fn deserialize_in(reader: &mut memorypack::MemoryPackReader<'_>, arena: &#lifetime #arena_type) -> Result<Self, memorypack::MemoryPackError> {
                #body
            }
        }
    }
}

fn add_field_deserialization_bounds(
    generics: &mut Generics,
    input: &DeriveInput,
    trait_path: &TokenStream
) {
    if input.generics.lifetimes().next().is_none() {
        return;
    }

    let predicates = input_fields(input)
        .into_iter()
        .filter(|field| {
            FieldAttributes::for_field(field)
                .formatter
                .is_none()
        })
        .map(|field| prepare_field_deserialization_bound(field, trait_path));

    generics
        .make_where_clause()
        .predicates
        .extend(predicates);
}

fn input_fields(input: &DeriveInput) -> Vec<&Field> {
    match &input.data {
        Data::Struct(data) => data.fields.iter().collect(),
        Data::Enum(data) => data
            .variants
            .iter()
            .flat_map(|variant| &variant.fields)
            .collect(),
        Data::Union(data) => data.fields.named.iter().collect()
    }
}

fn requires_default_traits(input: &DeriveInput, parameter: &Ident) -> bool {
    let fields = input_fields(input);

    if fields.iter().all(|field| {
        FieldAttributes::for_field(field)
            .formatter
            .is_none()
    }) {
        return true;
    }

    let mut usage = TypeParameterUsage { parameter, used: false };

    for field in fields.iter().filter(|field| {
        !should_skip_field(field)
            && FieldAttributes::for_field(field)
                .formatter
                .is_none()
    }) {
        usage.visit_type(&field.ty);
    }

    usage.used
}

fn prepare_field_deserialization_bound(field: &Field, trait_path: &TokenStream) -> WherePredicate {
    let field_type = &field.ty;

    if should_skip_field(field) {
        return parse_quote!(#field_type: Default);
    }

    parse_quote!(#field_type: #trait_path)
}

fn prepare_input_lifetime(generics: &Generics, name: &str) -> Lifetime {
    let mut lifetime_name = name.to_owned();

    while generics
        .lifetimes()
        .any(|parameter| parameter.lifetime.to_string() == lifetime_name)
    {
        lifetime_name.push('_');
    }

    Lifetime::new(&lifetime_name, Span::call_site())
}

fn prepare_arena_type(generics: &Generics) -> Ident {
    let mut name = "MemoryPackAllocator".to_owned();

    while generics
        .params
        .iter()
        .any(|parameter| match parameter {
            GenericParam::Type(parameter) => parameter.ident == name,
            GenericParam::Const(parameter) => parameter.ident == name,
            GenericParam::Lifetime(_) => false
        })
    {
        name.push('_');
    }

    Ident::new(&name, Span::call_site())
}
