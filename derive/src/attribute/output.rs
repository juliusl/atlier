
use syn::punctuated::Punctuated;
use syn::{Attribute, Ident, Result, Token};
use syn::parse::{Parse, ParseStream};
use quote::{format_ident, quote, quote_spanned};
use inflector::Inflector;


// Parse the output attribute 
// #[output(name, type)]

pub struct Output {
    ident: syn::Ident,
    ty: syn::Ident,
}

impl Parse for Output {
    fn parse(input: ParseStream) -> Result<Self> {
        let params = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;

        let params: Vec<Ident> = params.into_iter().collect();

        Ok(Output {
            ident: params[0].clone(),
            ty: params[1].clone(),
        })
    }
}

impl Output {
    pub fn format_output_fn_def(&self, param_defs: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let ty = &self.ty;

        quote! {
            fn #ident(#param_defs) -> Option<#ty>;
        }
    }

    pub fn format_id_field(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let ident = format_ident!("{}", ident.to_string().to_foreign_key());
        let next_ident = format_ident!("{}_next", ident);

        quote! {
            pub #ident: N::OutputId,
            pub #next_ident: Option<N::NodeId>,
        }
    }

    pub fn format_new_id_expr(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let ident = format_ident!("{}", ident.to_string().to_foreign_key());
        let next_ident = format_ident!("{}_next", ident);

        quote! {
            #ident: node.next_output_id(),
            #next_ident: None,
        }
    }

    pub fn format_transition(&self, name: &Ident, params: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let ident = &self.ident;

        let enum_name = format_ident!("{}Output", name, );
        let enum_path = format_ident!("{}", ident.to_string().to_title_case());
        let invoke_name = format_ident!("{}Outputs", name);

        quote_spanned! {ident.span()=>
            #enum_name::#enum_path(Some(v)) => {
                let next = <#name as #invoke_name>::#ident(#params);
                if let Some(next) = next {
                    if next != v {
                        return #enum_name::#enum_path(Some(next))
                    }
                }

                #enum_name::#enum_path(Some(v))
            },
            #enum_name::#enum_path(None) => #enum_name::#enum_path(<#name as #invoke_name>::#ident(#params)),
        }
    }

    pub fn format_enum_ident(&self) -> proc_macro2::TokenStream {
        let ident = format_ident!("{}", self.ident.to_string().to_title_case());
        let ty= &self.ty;

        quote! {
            #ident(Option<#ty>),
        }
    }

    pub fn format_artifact_expression(&self) -> proc_macro2::TokenStream {
        let ident_str = format!("{}", self.ident);
        let field_name = format_ident!("{}", format!("{}", self.ident).to_foreign_key());
        let ty= &self.ty;

        quote! {
            Artifact::<N>::new_output(
                self.#field_name.clone(), 
                #ident_str.to_string(), 
                #ty::default().type_id(),
            ),
        }
    }
}

pub fn parse_output_attribute(attr: &Attribute) -> Result<Output> {
    let params: Output = attr.parse_args()?;

    Ok(params)
}
