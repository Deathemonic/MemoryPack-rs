use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::generation::helpers::{add_unmanaged_bounds, has_repr_c};
use crate::generation::unmanaged;

pub fn generate_impl(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let mut generics = input.generics.clone();
    add_unmanaged_bounds(&mut generics, input);
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let (size, alignment) = match &input.data {
        Data::Struct(data) if has_repr_c(&input.attrs) => {
            (unmanaged::size_hint(data), unmanaged::alignment(data))
        }
        _ => (quote! { ::core::mem::size_of::<i32>() }, quote! { ::core::mem::align_of::<i32>() })
    };

    quote! {
        impl #impl_generics memorypack::MemoryPackFixedSize for #name #type_generics #where_clause {
            const SIZE: usize = #size;
            const ALIGNMENT: usize = #alignment;
        }

        impl #impl_generics memorypack::MemoryPackNonNullableValue for #name #type_generics #where_clause {}
    }
}

pub fn generate_nullable_serialize() -> TokenStream {
    quote! {
        fn serialize_nullable(value: Option<&Self>, writer: &mut memorypack::MemoryPackWriter) -> Result<(), memorypack::MemoryPackError> {
            <Self as memorypack::MemoryPackFixedSize>::serialize_nullable_value(value, writer)
        }

        fn nullable_size_hint(_value: Option<&Self>) -> usize {
            <Self as memorypack::MemoryPackFixedSize>::NULLABLE_SIZE
        }
    }
}
