use proc_macro::TokenStream;

mod property;
mod interop_enum;

macro_rules! propagate_err {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return err.to_compile_error().into(),
        }
    };
}

pub(crate) use propagate_err;

#[proc_macro_derive(Property, attributes(property))]
pub fn property(item: TokenStream) -> TokenStream {
    property::make_derive(item)
}

#[proc_macro_derive(GenerateEvent)]
pub fn generate_event(input: TokenStream) -> TokenStream {
    interop_enum::make_event(input)
}

