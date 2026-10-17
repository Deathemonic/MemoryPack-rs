use proc_macro2::TokenStream;
use syn::Data;

use crate::generation::version_tolerant::{generate_deserialize, generate_serialize};

pub fn generate_circular_serialize(data: &Data) -> TokenStream { generate_serialize(data, true) }

pub fn generate_circular_deserialize(data: &Data) -> TokenStream {
    generate_deserialize(data, true)
}
