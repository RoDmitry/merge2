//! A derive macro for the [`merge2::Merge`][] trait.

extern crate proc_macro;

use manyhow::bail;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::Token;

struct Field {
    name: syn::Member,
    span: proc_macro2::Span,
    attrs: FieldAttrs,
}

#[derive(Default)]
struct FieldAttrs {
    skip: bool,
    strategy: Option<syn::Path>,
}

enum FieldAttr {
    Skip,
    Strategy(syn::Path),
}

#[proc_macro_derive(Merge, attributes(merge))]
pub fn merge_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    manyhow::function!(input, impl_merge)
}

fn impl_merge(input: syn::DeriveInput, dummy: &mut TokenStream) -> manyhow::Result<TokenStream> {
    let name = &input.ident;
    let default_strategy = FieldAttrs::from(input.attrs.iter());

    *dummy = quote! {
        impl ::merge2::Merge for #name {
            fn merge(&mut self, other: &mut Self) {
                unimplemented!()
            }
        }
    };

    if let syn::Data::Struct(syn::DataStruct { ref fields, .. }) = input.data {
        Ok(impl_merge_for_struct(name, fields, default_strategy))
    } else {
        bail!("merge::Merge can only be derived for structs")
    }
}

fn impl_merge_for_struct(
    name: &syn::Ident,
    fields: &syn::Fields,
    default_strategy: FieldAttrs,
) -> TokenStream {
    let assignments = gen_assignments(fields, default_strategy);

    quote! {
        impl ::merge2::Merge for #name {
            fn merge(&mut self, other: &mut Self) {
                #assignments
            }
        }
    }
}

fn gen_assignments(fields: &syn::Fields, default_strategy: FieldAttrs) -> TokenStream {
    let fields = fields.iter().enumerate().map(Field::from);
    let assignments = fields
        .filter(|f| !f.attrs.skip)
        .map(|f| gen_assignment(&f, &default_strategy));
    quote! {
        #( #assignments )*
    }
}

fn gen_assignment(field: &Field, default_strategy: &FieldAttrs) -> TokenStream {
    use syn::spanned::Spanned;

    let name = &field.name;
    if let Some(strategy) = &field.attrs.strategy {
        quote_spanned!(strategy.span()=> #strategy(&mut self.#name, &mut other.#name);)
    } else if let Some(default) = &default_strategy.strategy {
        quote_spanned!(default.span()=> #default(&mut self.#name, &mut other.#name);)
    } else {
        quote_spanned!(field.span=> ::merge2::Merge::merge(&mut self.#name, &mut other.#name);)
    }
}

impl From<(usize, &syn::Field)> for Field {
    fn from(data: (usize, &syn::Field)) -> Self {
        use syn::spanned::Spanned;

        let (index, field) = data;
        Field {
            name: if let Some(ident) = &field.ident {
                syn::Member::Named(ident.clone())
            } else {
                syn::Member::Unnamed(index.into())
            },
            span: field.span(),
            attrs: field.attrs.iter().into(),
        }
    }
}

impl FieldAttrs {
    fn apply(&mut self, attr: FieldAttr) {
        match attr {
            FieldAttr::Skip => self.skip = true,
            FieldAttr::Strategy(path) => self.strategy = Some(path),
        }
    }
}

impl<'a, I: Iterator<Item = &'a syn::Attribute>> From<I> for FieldAttrs {
    fn from(iter: I) -> Self {
        let mut field_attrs = Self::default();

        for attr in iter {
            if !attr.path().is_ident("merge") {
                continue;
            }

            let parser = syn::punctuated::Punctuated::<FieldAttr, Token![,]>::parse_terminated;
            for attr in attr.parse_args_with(parser).unwrap() {
                field_attrs.apply(attr);
            }
        }

        field_attrs
    }
}

impl syn::parse::Parse for FieldAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let name: syn::Ident = input.parse()?;
        if name == "skip" {
            // TODO check remaining stream
            Ok(FieldAttr::Skip)
        } else if name == "strategy" {
            let _: Token![=] = input.parse()?;
            let path: syn::Path = input.parse()?;
            Ok(FieldAttr::Strategy(path))
        } else {
            bail!(name, "Unexpected attribute: {}", name)
        }
    }
}
