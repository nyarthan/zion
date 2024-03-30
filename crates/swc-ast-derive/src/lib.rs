use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, Ident, ImplGenerics, TypeGenerics,
    Variant,
};

#[proc_macro_derive(ToAstStruct)]
pub fn derive_to_ast_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => generate_struct_impl(
                &fields_named,
                struct_name,
                impl_generics,
                ty_generics,
                where_clause,
            ),
            _ => unimplemented!(),
        },
        Data::Enum(data_enum) => generate_enum_impl(
            &data_enum.variants,
            struct_name,
            impl_generics,
            ty_generics,
            where_clause,
        ),
        _ => unimplemented!(),
    }
}

fn generate_struct_impl(
    fields_named: &FieldsNamed,
    struct_name: &Ident,
    impl_generics: ImplGenerics,
    ty_generics: TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
) -> TokenStream {
    let field_accesses = fields_named.named.iter().map(|f| {
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

fn generate_enum_impl(
    variants: &syn::punctuated::Punctuated<Variant, syn::token::Comma>,
    struct_name: &Ident,
    impl_generics: ImplGenerics,
    ty_generics: TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
) -> TokenStream {
    let variant_matches = variants.iter().map(|v| {
        let ident = &v.ident;
        let variant_str = ident.to_string().to_case(Case::Camel);
        quote! {
            #struct_name::#ident => stringify!(#variant_str),
        }
    });

    quote! {
        impl #impl_generics ToAst for #struct_name #ty_generics #where_clause {
            fn to_ast_node(&self) -> Expr {
                let variant_str = match self {
                    #(#variant_matches)*
                };

                Expr::Lit(Lit::Str(Str {
                    span: DUMMY_SP,
                    value: variant_str.replace("\"", "").into(),
                    raw: None
                }))
            }
        }
    }
    .into()
}
