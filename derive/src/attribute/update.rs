use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, Ident, Result, Token};

// Parse the output attribute
// #[output(name, type)]

pub struct Update {
    ident: syn::Ident,
    ty: syn::Ident,
}

impl Parse for Update {
    fn parse(input: ParseStream) -> Result<Self> {
        let params = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;

        let params: Vec<Ident> = params.into_iter().collect();

        Ok(Update {
            ident: params[0].clone(),
            ty: params[1].clone(),
        })
    }
}

impl Update {
    pub fn format_component(&self, runtime_ident: &Ident) -> proc_macro2::TokenStream {
        let node_ident = format_ident!("{}Node", self.ident);
        let ty = &self.ty;
        let system_data_ident = format_ident!("Updating{}SystemData", self.ident);
        let runtime_updater = format_ident!("{}{}Updater", self.ident, runtime_ident);

        quote! {
            #[derive(SystemData)]
            pub struct #system_data_ident<'a> {
                resources: specs::prelude::Write<'a, ContentStore<#runtime_ident>>,
                store: specs::prelude::WriteStorage<'a, #ty>,
                nodes: specs::prelude::WriteStorage<'a, #node_ident<#runtime_ident>>,
            }

            impl<'a> specs::prelude::System<'a> for #runtime_updater {
                type SystemData = #system_data_ident<'a>;

                fn run(&mut self, mut data: Self::SystemData) {
                    let resource = data.resources.deref_mut();
                    for (d, n) in (&mut data.store, &mut data.nodes).join() {
                        self.update(resource, d, n);
                    }
                }
            }
        }
    }
}


pub fn parse_update_attribute(attr: &Attribute) -> Result<Update> {
    let params: Update = attr.parse_args()?;

    Ok(params)
}
