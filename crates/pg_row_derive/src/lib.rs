use syn::{parse_macro_input, Data, DeriveInput, Fields};
use proc_macro::TokenStream;
use quote::quote;



#[proc_macro_derive(RowFrom)]
pub fn derive_row_from(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => {
                return syn::Error::new_spanned(
                    name,
                    "RowFrom requires named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                name,
                "RowFrom can only be derived for structs",
            )
            .to_compile_error()
            .into();
        }
    };

    let fields = fields.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        quote! {
            #name: row.get::<_, #ty>(stringify!(#name))
        }
    });

    quote! {
        impl From<tokio_postgres::Row> for #name {
            fn from(row: tokio_postgres::Row) -> Self {
                Self {
                    #(#fields),*
                }
            }
        }

        impl #name {
            pub fn from_rows(
                rows: Vec<tokio_postgres::Row>,
            ) -> Vec<Self> {
                rows.into_iter().map(Self::from).collect()
            }
        }
    }
    .into()
}
