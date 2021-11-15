extern crate proc_macro;
use core::panic;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, TokenStreamExt};
use syn::{spanned::Spanned, DeriveInput, FieldsNamed, Ident, Type};
mod attribute;

#[proc_macro_derive(Transition, attributes(output))]
pub fn derive_transition(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed: syn::DeriveInput = syn::parse_macro_input!(input);

    let node_data: ParsedStruct = ParsedStruct::from(parsed);

    node_data.generate_transition_code()
}

#[proc_macro_derive(Node, attributes(output))]
pub fn derive_node(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed: syn::DeriveInput = syn::parse_macro_input!(input);

    let node_data: ParsedStruct = ParsedStruct::from(parsed);

    node_data.generate_node_code()
}

#[proc_macro_derive(Renderer, attributes(render))]
// First, you should implement the Renderer trait with the type of Data
// that you would like to render. 
// Then you can add this derive attribute along with a `#[render()]` attribute.
// The render attributes will generate the data pipeline for the Transition struct, which you specify in the first argument
// the second argument is the component type you would like passed to your render function.
pub fn derive_render(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed: syn::DeriveInput = syn::parse_macro_input!(input);

    let ident = &parsed.ident;

    // Parse each #[output(..)] attribute defined on the Transition
    let attrs = &parsed.attrs;
    let renders: Vec<TokenStream> = attrs
              .into_iter()
              .filter(|a| a.path.is_ident("render"))
              .map(|a| attribute::render::parse_render_attribute(&a))
              .filter(|o| o.is_ok())
              .map(|o| match o {
                  Ok(o) => o.format_component(ident),
                  _ => unreachable!(),
              })
              .collect();

    let gen = quote! {
        #( #renders )*
    };

    gen.into()
}

// Inner type to have access to the underlying data
struct ParsedStruct {
    name: Ident,
    fields: FieldsNamed,
    output_attrs: Vec<attribute::output::Output>,
}

impl ParsedStruct {
    // parses the fields inside of the struct into the inputs and attribute
    fn parse_fields(
        &self,
        on_input: fn(Type, Ident, Ident) -> TokenStream,
        on_attr: fn(Type, Ident) -> TokenStream,
    ) -> Vec<TokenStream> {
        let fields = &self.fields;

        fields
            .named
            .iter()
            .map(|f| {
                let ident =
                    format_ident!("{}", f.ident.as_ref().unwrap().to_string().to_foreign_key());
                let via_ident = format_ident!("{}_via", f.ident.as_ref().unwrap());

                let gen = match &f.ty {
                    syn::Type::Path(v)
                        if v.qself.is_some()
                            && v.path
                                .clone()
                                .segments
                                .iter()
                                .any(|f| f.ident.to_string().starts_with("Option<")) =>
                    {
                        let parse_input = on_input(f.ty.clone(), ident, via_ident);
                        quote_spanned!(f.span()=> #parse_input)
                    }
                    _ => {
                        let parse_attr = on_attr(f.ty.clone(), ident);
                        quote_spanned!(f.span()=> #parse_attr)
                    }
                };

                gen
            })
            .collect()
    }

    // This generates the *Node struct
    // Each optional field is assigned an Node::InputId
    // Otherwise the field is assigned an Node::AttributeId
    // Finally, each #[output] is assigned a Node::OutputId
    fn generate_node_code(&self) -> proc_macro::TokenStream {
        let node_struct = format_ident!("{}Node", &self.name);

        let field_defs = self.parse_fields(
            |_, ident, via| {
                quote! {
                    pub #ident: N::InputId, pub #via: Option<N::NodeId>
                }
            },
            |_, ident| {
                quote! {
                    pub #ident: N::AttributeId
                }
            },
        );

        let new_fields = self.parse_fields(
            |_, ident, via| {
                quote! {
                    #ident: node.next_input_id(), #via: None
                }
            },
            |_, ident| {
                quote! {
                    #ident: node.next_attribute_id()
                }
            },
        );

        let input_artifacts = self.parse_fields(
            |ty, ident, _| {
                let ident_str = format!("{}", ident);
                let field_name = format_ident!("{}", format!("{}", ident).to_foreign_key());

                quote! {
                    Artifact::<Self::N>::new_input(
                        self.#field_name.clone(),
                        #ident_str.to_string(),
                        #ty::default().type_id(),
                    ),
                }
            },
            |_, _| {
                quote! {}
            },
        );

        let attribute_artifacts = self.parse_fields(
            |_, _, _| {
                quote! {}
            },
            |ty, ident| {
                let ident_str = format!("{}", ident);
                let field_name = format_ident!("{}", format!("{}", ident).to_foreign_key());
                quote! {
                        Artifact::<Self::N>::new_attribute(
                            self.#field_name.clone(),
                            #ident_str.to_string(),
                            #ty::default().type_id(),
                        ),
                }
            },
        );

        let mut outputs = quote!();
        let mut outputs_new = quote!();
        let mut output_artifact = quote!();

        for o in self.output_attrs.iter() {
            let output_field = o.format_id_field();
            outputs.append_all(output_field);

            let output_new_expr = o.format_new_id_expr();
            outputs_new.append_all(output_new_expr);

            let output_artifact_expr = o.format_artifact_expression();
            output_artifact.append_all(output_artifact_expr);
        }

        let gen = quote! {
            #[derive(Debug, Hash, Eq, PartialEq)]
            pub struct #node_struct<N>
            where
                N: Node + Hash + Eq + PartialEq + Sync,
            {
                node_id: N::NodeId,
                #( #field_defs, )*
                #outputs
            }

            impl<N> From<N> for #node_struct<N>
            where
                N: Node + Hash + Eq + PartialEq + Sync,
            {
                fn from(mut node: N) -> Self {
                    Self {
                        node_id: node.next_node_id(),
                        #( #new_fields, )*
                        #outputs_new
                    }
                }
            }


            impl<N> State for #node_struct<N>
            where
                N: Node + Hash + Eq + PartialEq + Sync,
            {
                type N = N;
                type Inputs = ArtifactCollection::<Self::N>;
                type Outputs = ArtifactCollection::<Self::N>;
                type Attributes = ArtifactCollection::<Self::N>;

                fn get_nodeid(&self) -> N::NodeId {
                    self.node_id.clone()
                }

                fn get_inputs(&self) -> Self::Inputs {
                    ArtifactCollection::<Self::N>{
                        elems: vec![
                            #( #input_artifacts )*
                        ]}
                }

                fn get_outputs(&self) -> Self::Outputs {
                    ArtifactCollection::<Self::N>{
                        elems: vec![
                             #output_artifact
                        ]}
                }

                fn get_attributes(&self) -> Self::Attributes {
                    ArtifactCollection::<Self::N>{
                        elems: vec![
                            #( #attribute_artifacts )*
                        ]}
                }
            }
        };

        proc_macro::TokenStream::from(gen)
    }

    // impl From<Editor> for AddNode<'static, Editor> {
    //     fn from(mut e: Editor) -> Self {
    //         AddNode::<Editor> {
    //             node_id: e.next_node_id(),
    //             lhs_id: e.next_attribute_id(),
    //             rhs_id: e.next_attribute_id(),
    //             sum_id: e.next_output_id(),
    //             sum_id_next: None,
    //             display_id: e.next_output_id(),
    //             display_id_next: None,
    //         }
    //     }
    // }

    // This generates the Transition impl, *Outputs trait, and *Output enum
    fn generate_transition_code(&self) -> proc_macro::TokenStream {
        let outputs_trait = format_ident!("{}Outputs", &self.name);
        let fields = &self.fields;
        let name = &self.name;

        // These variables are the beginning of the streams we will be creating
        let mut output_fns = quote! {};
        let mut transition_impls = quote! {};
        let mut enum_items = quote! {};
        let enum_name = format_ident!("{}Output", name);

        // This loop generates code for each of the above streams
        for o in self.output_attrs.iter() {
            // This creates the trait that the user can implement for their functionality
            let fn_params = fields.named.iter();
            let fn_quote = o.format_output_fn_def(quote!(#( #fn_params ),*));

            output_fns.append_all(fn_quote);

            // This generates the transition matches, to call the correct output fn
            let fields_ident = fields.named.clone();
            let invoke_params = fields_ident.iter().map(|f| &f.ident);
            let transition_impl = o.format_transition(&name, quote!(#(self.#invoke_params),*));

            transition_impls.append_all(transition_impl);

            // This generates the Output enum's
            let enum_name = o.format_enum_ident();
            enum_items.append_all(enum_name)
        }

        // Finally, we compose the resulting streams into it's final output form
        let gen = quote! {
            #[derive(Clone, Debug)]
            pub enum #enum_name {
                #enum_items
            }

            impl Transition for #name {
                type Output = #enum_name;

                fn transition(&mut self, select_output: Self::Output) -> Self::Output {
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
}

impl From<DeriveInput> for ParsedStruct {
    // From the input that derive receives create a ParsedStruct
    fn from(parsed: DeriveInput) -> Self {
        let name = &parsed.ident;

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
        let attrs = &parsed.attrs;
        let outputs: Vec<attribute::output::Output> = attrs
            .into_iter()
            .filter(|a| a.path.is_ident("output"))
            .map(|a| attribute::output::parse_output_attribute(&a))
            .filter(|o| o.is_ok())
            .map(|o| match o {
                Ok(o) => o,
                _ => unreachable!(),
            })
            .collect();

        ParsedStruct {
            name: name.clone(),
            fields: fields.clone(),
            output_attrs: outputs,
        }
    }
}
