use quote::quote;

#[inline]
pub fn generate_enum_serialize() -> proc_macro2::TokenStream {
    quote! {
        writer.write_i32(*self as i32)?;
    }
}

pub fn generate_enum_deserialize_unsafe() -> proc_macro2::TokenStream {
    quote! {
        let value = reader.read_i32()?;
        Ok(unsafe { std::mem::transmute(value) })
    }
}

pub fn generate_enum_deserialize_safe(data_enum: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let discriminant = &variant.discriminant.as_ref().unwrap().1;

        quote! {
            #discriminant => Ok(Self::#variant_name),
        }
    });

    quote! {
        let value = reader.read_i32()?;
        match value {
            #(#variants)*
            _ => Err(memorypack::MemoryPackError::DeserializationError(
                format!("Invalid discriminant {} for enum {}", value, stringify!(Self))
            ))
        }
    }
}

#[inline]
pub fn generate_transparent_serialize() -> proc_macro2::TokenStream {
    quote! {
        writer.write_i32(self.0)?;
    }
}

#[inline]
pub fn generate_transparent_deserialize() -> proc_macro2::TokenStream {
    quote! {
        Ok(Self(reader.read_i32()?))
    }
}

pub fn generate_flags_impls(name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl #name {
            #[inline]
            pub const fn contains(self, other: #name) -> bool {
                (self.0 & other.0) == other.0
            }

            #[inline]
            pub const fn is_empty(self) -> bool {
                self.0 == 0
            }
        }

        impl std::ops::BitOr for #name {
            type Output = Self;
            #[inline]
            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }

        impl std::ops::BitAnd for #name {
            type Output = Self;
            #[inline]
            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }

        impl std::ops::BitXor for #name {
            type Output = Self;
            #[inline]
            fn bitxor(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }

        impl std::ops::Not for #name {
            type Output = Self;
            #[inline]
            fn not(self) -> Self {
                Self(!self.0)
            }
        }
    }
}
