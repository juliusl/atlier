use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, Ident, Result, Token};

// Parse the output attribute
// #[output(name, type)]

pub struct Render {
    ident: syn::Ident,
    ty: syn::Ident,
}

impl Parse for Render {
    fn parse(input: ParseStream) -> Result<Self> {
        let params = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;

        let params: Vec<Ident> = params.into_iter().collect();

        Ok(Render {
            ident: params[0].clone(),
            ty: params[1].clone(),
        })
    }
}

impl Render {
    pub fn format_component(&self, runtime_ident: &Ident) -> proc_macro2::TokenStream {
        let node_ident = format_ident!("{}Node", self.ident);
        let ty = &self.ty;
        quote! {
               // TODO: Derived
            impl specs::prelude::Component for #node_ident<#runtime_ident, #ty> {
                type Storage = specs::prelude::DenseVecStorage<Self>;
            }

            impl<'a> specs::prelude::System<'a> for #runtime_ident {
                type SystemData = specs::prelude::ReadStorage<'a, #node_ident<#runtime_ident, #ty>>;

                fn run(&mut self, data: Self::SystemData) {
                    for d in data.join() {
                        self.render(d);
                    }
                }
            }
        }
    }
}

pub fn parse_render_attribute(attr: &Attribute) -> Result<Render> {
    let params: Render = attr.parse_args()?;

    Ok(params)
}
