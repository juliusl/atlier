extern crate proc_macro;
use core::panic;
use quote::{format_ident, quote, TokenStreamExt};
mod transition;

#[proc_macro_derive(Transition, attributes(output))]
pub fn derive_invoke(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed: syn::DeriveInput = syn::parse_macro_input!(input);

    let name = parsed.ident;

    // Create a public trait that the user can use to implement their output methods
    // This also gives us a way to refer to these methods programatically
    let outputs_trait = format_ident!("{}Outputs", name);

    // Parse the fields in this struct
    // Structs are only supported for now, but enums and tuples could be nice to have as well
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = parsed.data
    {
        fields
    } else {
        panic!("Currently, only deriving from a Struct type is supported")
    };

    // Parse each #[output(..)] attribute defined on the Transition
    let outputs: Vec<transition::output::Output> = parsed
        .attrs
        .into_iter()
        .filter(|a| a.path.is_ident("output"))
        .map(|a| transition::output::parse_output_attribute(&a))
        .filter(|o| o.is_ok())
        .map(|o| match o {
            Ok(o) => o,
            _ => unreachable!(),
        })
        .collect();


    let mut output_fns = quote! {};
    let mut transition_impls = quote! {};
    let mut enum_items = quote!{};
    let enum_name = format_ident!("{}Output", name);

    for mut o in outputs {
        // This creates the trait that the user can implement for their functionality
        let fn_params = fields.named.iter();
        let fn_quote = o.format_output_fn_def(quote!(#( #fn_params ),*));

        output_fns.append_all(fn_quote);

        // This generates the transition matches, to call the correct output fn
        let fields_ident = fields.named.clone();
        let invoke_params = fields_ident.iter().map(|f| &f.ident);
        let transition_impl = o.format_transition(&name, quote!(#(data.#invoke_params),*));

        transition_impls.append_all(transition_impl);

        // This generates the Output enum's 
        let enum_name = o.format_enum_ident();
        enum_items.append_all(enum_name)
    }

    let gen = quote! {
        pub enum #enum_name {
            #enum_items
        }

        impl Transition for #name {
            type Data = #name;
            type Output = #enum_name;

            fn transition(data: Self::Data, select_output: Self::Output) -> Self::Output {
                match select_output {
                    #transition_impls
                }
            }
        }

        // When implemented by the user, these output functions
        // will be called when transitioning to an output
        trait #outputs_trait {
            #output_fns
        }
    };

    proc_macro::TokenStream::from(gen)
}
