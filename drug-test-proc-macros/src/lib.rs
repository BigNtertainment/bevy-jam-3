use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Temporary)]
pub fn temporary(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_temporary(&ast)
}

fn impl_temporary(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Temporary for #name {
            fn get_timer(&mut self) -> &mut Timer {
                &mut self.timer
            }
        }
    };

    gen.into()
}
