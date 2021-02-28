use proc_macro::TokenStream;

struct Task;

trait TaskTrait {}

impl TaskTrait for Task {}

// Source: https://github.com/SergioBenitez/Rocket/blob/337e8843a47d4674147462e109e971f56dc5d50c/core/codegen/src/lib.rs
// https://github.com/SergioBenitez/Rocket/blob/337e8843a47d4674147462e109e971f56dc5d50c/core/codegen/src/attribute/async_entry.rs

// mod async_entry;

// fn task_back(args: TokenStream, input: TokenStream) -> TokenStream {
//     args
// }

// macro_rules! emit {
//     ($tokens:expr) => {
//         $tokens.into()
//     };
// }

// #[proc_macro_attribute]
// pub fn task(_args: TokenStream, _input: TokenStream) -> TokenStream {
//     emit!(task_back)
//     // pub fn task_hi() {
//     //     println!("Ok, world!");
//     // }
//     // println!("Hello, how are you?");
// }

// trait EntryAttr {
//     /// Whether the attribute requires the attributed function to be `async`.
//     const REQUIRES_ASYNC: bool;

//     /// Return a new or rewritten function, using block as the main execution.
//     fn function(f: &mut syn::ItemFn) -> Result<TokenStream>;
// }

// fn task_creator(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> TokenStream {
//     println(
//     args
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
