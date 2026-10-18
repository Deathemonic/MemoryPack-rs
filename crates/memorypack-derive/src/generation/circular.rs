use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::generation::helpers::add_formatter_bounds;
use crate::generation::version_tolerant::{
    generate_deserialize,
    generate_graph_deserialize,
    generate_graph_serialize,
    generate_serialize
};

pub fn generate_circular_serialize(data: &Data) -> TokenStream { generate_serialize(data, true) }

pub fn generate_circular_deserialize(data: &Data) -> TokenStream {
    generate_deserialize(data, true)
}

pub fn generate_graph_impl(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let mut generics = input.generics.clone();
    add_formatter_bounds(&mut generics, input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let serialize = generate_graph_serialize(&input.data);
    let deserialize = generate_graph_deserialize(&input.data);

    quote! {
        impl #impl_generics memorypack::MemoryPackCircular for #name #ty_generics #where_clause {
            fn serialize_circular(&self, writer: &mut memorypack::MemoryPackWriter, reference_id: u32)
                -> Result<(), memorypack::MemoryPackError> {
                #serialize
                Ok(())
            }

            fn deserialize_circular(reader: &mut memorypack::MemoryPackReader, lengths: &[usize])
                -> Result<Self, memorypack::MemoryPackError> {
                #deserialize
            }
        }
    }
}
