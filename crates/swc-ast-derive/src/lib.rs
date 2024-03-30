use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ToAstStruct)]
pub fn derive_to_ast_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    let field_accesses = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            PropOrSpread::Prop(
                Prop::KeyValue(KeyValueProp {
                    key: PropName::Ident(Ident {
                        span: DUMMY_SP,
                        sym: stringify!(#name).into(),
                        optional: false,
                    }),
                    value: self.#name.to_ast_node().into(),
                })
                .into(),
            )
        }
    });

    quote! {
        impl #impl_generics ToAst for #struct_name #ty_generics #where_clause {
            fn to_ast_node(&self) -> Expr {
                Expr::Object(ObjectLit {
                    span: DUMMY_SP,
                    props: vec![
                        #(#field_accesses),*
                    ],
                })
            }
        }
    }
    .into()
}
