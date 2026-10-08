use quote::quote;
use syn::{DataStruct, Fields};

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

pub fn serialize(data: &DataStruct) -> proc_macro2::TokenStream {
    let fields = field_layout(&data.fields);
    let writes = fields.iter().map(|(ty, access)| {
        quote! {
            let alignment = std::mem::align_of::<#ty>();
            offset = (offset + alignment - 1) & !(alignment - 1);
            while writer.len() < start + offset { writer.write_u8(0)?; }
            memorypack::MemoryPackSerialize::serialize(&self.#access, writer)?;
            offset += std::mem::size_of::<#ty>();
            max_alignment = max_alignment.max(alignment);
        }
    });
    quote! {
        let start = writer.len();
        let mut offset = 0_usize;
        let mut max_alignment = 1_usize;
        #(#writes)*
        let size = (offset + max_alignment - 1) & !(max_alignment - 1);
        while writer.len() < start + size { writer.write_u8(0)?; }
    }
}

pub fn deserialize(data: &DataStruct) -> proc_macro2::TokenStream {
    let fields = field_layout(&data.fields);
    let reads = fields.iter().enumerate().map(|(index, (ty, _))| {
        let var = syn::Ident::new(&format!("field_{index}"), proc_macro2::Span::call_site());
        quote! {
            let alignment = std::mem::align_of::<#ty>();
            offset = (offset + alignment - 1) & !(alignment - 1);
            if offset > consumed { reader.skip(offset - consumed)?; }
            let #var = memorypack::MemoryPackDeserialize::deserialize(reader)?;
            offset += std::mem::size_of::<#ty>();
            consumed = offset;
            max_alignment = max_alignment.max(alignment);
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
        let mut max_alignment = 1_usize;
        #(#reads)*
        let size = (offset + max_alignment - 1) & !(max_alignment - 1);
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
        #(let alignment = std::mem::align_of::<#types>(); offset = (offset + alignment - 1) & !(alignment - 1); offset += std::mem::size_of::<#types>(); max_alignment = max_alignment.max(alignment);)*
        (offset + max_alignment - 1) & !(max_alignment - 1)
    }}
}
