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
        let system_data_ident = format_ident!("{}SystemData", self.ident);
        let runtime_renderer = format_ident!("{}{}Renderer", self.ident, runtime_ident);
        
        quote! {
            #[derive(SystemData)]
            pub struct #system_data_ident<'a> {
                resources: specs::prelude::Read<'a, ContentStore<#runtime_ident>>,
                store: specs::prelude::ReadStorage<'a, #ty>,
                nodes: specs::prelude::ReadStorage<'a, #node_ident<#runtime_ident>>,
            }

            impl specs::prelude::Component for #node_ident<#runtime_ident> {
                type Storage = specs::prelude::DenseVecStorage<Self>;
            }

            impl<'a> specs::prelude::System<'a> for #runtime_renderer{
                type SystemData = #system_data_ident<'a>;

                fn run(&mut self, data: Self::SystemData) {
                    for (d, n) in (&data.store, &data.nodes).join() {
                        self.render(&data.resources.deref(), d, n);
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
