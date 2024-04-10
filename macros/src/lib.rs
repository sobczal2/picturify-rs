extern crate proc_macro;

// #[proc_macro_derive(pipeline_run)]
// pub fn pipeline_run(item: TokenStream) -> TokenStream {
//     // Parse the input token stream into a syntax tree
//     let input = syn::parse_macro_input!(item as syn::DeriveInput);
// 
//     // Get the name of the struct where the macro is applied
//     let struct_name = &input.ident;
// 
//     // Generate the implementation of the `run` function within the struct
//     let expanded = quote! {
//         impl #struct_name {
//             pub fn run(&self, fast_image: FastImage) -> FastImage {
//                 self.processors.iter().fold(fast_image, |acc, processor| {
//                     processor.process(acc)
//                 })
//             }
//         }
//     };
// 
//     // Convert the syntax tree back into a token stream
//     expanded.into()
// }
