use std::any::type_name;
pub fn print_type_of_var<T>(_: &T) {
    println!("{}", type_name::<T>());
}