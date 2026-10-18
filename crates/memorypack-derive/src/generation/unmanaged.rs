use quote::quote;
use syn::{DataStruct, Fields};

pub fn serialize(data: &DataStruct) -> proc_macro2::TokenStream {
    let fields = field_layout(&data.fields);
    let writes = fields.iter().map(|(ty, access)| {
        quote! {
            let alignment = <#ty as memorypack::MemoryPackFixedSize>::ALIGNMENT;
            offset = (offset + alignment - 1) & !(alignment - 1);
            while writer.len() < start + offset { writer.write_u8(0)?; }
            memorypack::MemoryPackSerialize::serialize(&self.#access, writer)?;
            offset += <#ty as memorypack::MemoryPackFixedSize>::SIZE;
        }
    });

    quote! {
        let start = writer.len();
        let mut offset = 0_usize;
        #(#writes)*
        let size = <Self as memorypack::MemoryPackFixedSize>::SIZE;
        while writer.len() < start + size { writer.write_u8(0)?; }
    }
}

pub fn deserialize(data: &DataStruct) -> proc_macro2::TokenStream {
    let fields = field_layout(&data.fields);
    let reads = fields.iter().enumerate().map(|(index, (ty, _))| {
        let var = syn::Ident::new(&format!("field_{index}"), proc_macro2::Span::call_site());

        quote! {
            let alignment = <#ty as memorypack::MemoryPackFixedSize>::ALIGNMENT;
            offset = (offset + alignment - 1) & !(alignment - 1);
            if offset > consumed { reader.skip(offset - consumed)?; }
            let #var = memorypack::MemoryPackDeserialize::deserialize(reader)?;
            offset += <#ty as memorypack::MemoryPackFixedSize>::SIZE;
            consumed = offset;
        }
    });
    let values: Vec<_> = (0..fields.len())
        .map(|index| syn::Ident::new(&format!("field_{index}"), proc_macro2::Span::call_site()))
        .collect();
    let result = match &data.fields {
        Fields::Named(fields) => {
            let names = fields
                .named
                .iter()
                .map(|field| field.ident.as_ref().expect("named field"));
            let assignments = names
                .zip(values.iter())
                .map(|(name, value)| quote! { #name: #value });

            quote! { Self { #(#assignments),* } }
        }
        Fields::Unnamed(_) => quote! { Self(#(#values),*) },
        Fields::Unit => quote! { Self }
    };

    quote! {
        let mut offset = 0_usize;
        let mut consumed = 0_usize;
        #(#reads)*
        let size = <Self as memorypack::MemoryPackFixedSize>::SIZE;
        if size > consumed { reader.skip(size - consumed)?; }
        Ok(#result)
    }
}

pub fn size_hint(data: &DataStruct) -> proc_macro2::TokenStream {
    let types = field_layout(&data.fields)
        .into_iter()
        .map(|(ty, _)| ty);

    quote! {{
        let mut offset = 0_usize;
        let mut max_alignment = 1_usize;
        #(
            let alignment = <#types as memorypack::MemoryPackFixedSize>::ALIGNMENT;
            offset = offset.next_multiple_of(alignment);
            offset += <#types as memorypack::MemoryPackFixedSize>::SIZE;

            if alignment > max_alignment {
                max_alignment = alignment;
            }
        )*

        if offset == 0 { 1 } else { offset.next_multiple_of(max_alignment) }
    }}
}

pub fn alignment(data: &DataStruct) -> proc_macro2::TokenStream {
    let types = field_layout(&data.fields)
        .into_iter()
        .map(|(ty, _)| ty);

    quote! {{
        let mut alignment = 1_usize;

        #(
            if <#types as memorypack::MemoryPackFixedSize>::ALIGNMENT > alignment {
                alignment = <#types as memorypack::MemoryPackFixedSize>::ALIGNMENT;
            }
        )*

        alignment
    }}
}

fn field_layout(fields: &Fields) -> Vec<(&syn::Type, proc_macro2::TokenStream)> {
    match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let name = field.ident.as_ref().expect("named field");

                (&field.ty, quote! { #name })
            })
            .collect(),
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, field)| {
                let index = syn::Index::from(index);

                (&field.ty, quote! { #index })
            })
            .collect(),
        Fields::Unit => Vec::new()
    }
}
