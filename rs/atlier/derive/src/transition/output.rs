
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
    pub fn format_output_fn_def(&mut self, param_defs: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let ty = &self.ty;

        quote! {
            fn #ident(#param_defs) -> Option<#ty>;
        }
    }

    pub fn format_transition(&mut self, name: &Ident, params: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let ident = &self.ident;

        let enum_name = format_ident!("{}Output", name, );
        let enum_path = format_ident!("{}", ident.to_string().to_title_case());
        let invoke_name = format_ident!("{}Outputs", name);

        quote_spanned! {ident.span()=>
            #enum_name::#enum_path(Some(v)) => #enum_name::#enum_path(Some(v)),
            #enum_name::#enum_path(None) => #enum_name::#enum_path(<#name as #invoke_name>::#ident(#params)),
        }
    }

    pub fn format_enum_ident(&mut self) -> proc_macro2::TokenStream {
        let ident = format_ident!("{}", &self.ident.to_string().to_title_case());
        let ty= &self.ty;

        quote! {
            #ident(Option<#ty>),
        }
    }
}

pub fn parse_output_attribute(attr: &Attribute) -> Result<Output> {
    let params: Output = attr.parse_args()?;

    Ok(params)
}
