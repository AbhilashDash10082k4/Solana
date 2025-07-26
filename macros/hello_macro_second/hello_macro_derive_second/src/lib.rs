extern crate proc_macro;
extern crate quote;
use proc_macro::TokenStream;
use quote::quote;

//proc_macro_derive - tells the type of the macro that the func defines, HelloMacro -name for the macro that the fn hello_macro_derive... defines
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive_fn_procmac(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.

    //TokenStream -> syn -> some changed format on which operations can be performed -> quote -> TokenStream
    //TokenStream -> parse -> DeriveInput
    //unwrap - causes hello_macro_derive_fn_procmac to panic on error and not return Result Error type but return TokenStream
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    impl_hello_macro(&ast)
}

//annotated type - in main.rs file - #[derive(HelloMacro)] -here HelloMacro is the name of the custom macro to be used and not the trait although the trait and macro names are kept to be same for convention
/*DeriveInput {
    code...
    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
} */

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    //custom macro - prints the name of the annotated type on which it is implemented
    let name = &ast.ident;

    //quote - defines the rust code that we want to return/ want the custom macro to expand into by calling into method
    // stringify is used in order to convert name(expression type) into string(printable format)s
    let generated = quote! {
        impl HelloMacroTrait for #name {
            fn hello_macro_func() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}