use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, DeriveInput, Data};

#[proc_macro_derive(EnumStringName)]
pub fn enum_string_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let (enum_names, string_names) = match &ast.data {
        Data::Enum(d) => {
            (
                d.variants.iter().map(|v| &v.ident),
                d.variants.iter().map(|v| v.ident.to_string().to_ascii_lowercase()),
            )
        },
        _ => panic!("only for enum!"),
    };

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl EnumStringName for #name {
            fn string_name(&self) -> String {
                match self {
                    #(#name::#enum_names => #string_names),*
                }.to_owned()
            }
        }
    };

    println!("{}", expanded);

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
