use hello_macro_derive_second::HelloMacro;
//macro named HelloMacro

use hello_macro_second::HelloMacroTrait;
//trait the macro HelloMacro implements , this is brought into scope so that the macro gets implemented properly

#[derive(HelloMacro)]
struct Abhilash;
fn main() {
    Abhilash::hello_macro_func();
}
