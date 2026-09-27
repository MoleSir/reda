use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Fields};

#[proc_macro_attribute]
pub fn model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;
    let vis = &input.vis;
    let attrs = &input.attrs;

    let fields = match &input.fields {
        Fields::Named(named) => &named.named,
        _ => {
            return syn::Error::new_spanned(&input, "Expected a struct with named fields")
                .to_compile_error()
                .into()
        }
    };

    let original_fields = fields.iter();

    let expanded = quote! {
        #(#attrs)*
        #vis struct #name {
            #(#original_fields,)*

            pub attrs: std::collections::HashMap<String, crate::model::LibValue>,
            pub groups: Vec<crate::model::LibGroup>,
        }

        impl crate::model::Model for #name {
            fn attrs(&self) -> &std::collections::HashMap<String, crate::model::LibValue> {
                &self.attrs
            }

            fn groups(&self) -> &[crate::model::LibGroup] {
                &self.groups
            }
        }
    };

    TokenStream::from(expanded)
}
