extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;

//use syn;
use crate::parse;
use parse::*;

use quote::{quote, ToTokens};

pub fn emit_composed_errors(args: ComposeErrorsAttribute, declenum: DeclEnum) -> TokenStream {
    let ctrs = build_constructors(&args, &declenum);
    let new_enum = splice_constructors(&ctrs, &declenum);
    let new_froms = write_froms(&ctrs, &declenum);

    let new_items = std::iter::FromIterator::from_iter(vec![new_enum, new_froms]);
    eprintln!("new_items: {}", new_items);

    return new_items;
}

fn write_froms(new_ctrs: &Vec<(syn::Path, syn::Variant)>, declenum: &DeclEnum) -> TokenStream{

    let streams = new_ctrs.iter().map(|(p, ctr)| {
        let error_type_name = p;
        let target_type_name: syn::Ident = declenum.ident.clone();
        let variant_constructor_name = ctr.ident.clone();

        let from = quote! {
            impl std::convert::From<#error_type_name> for #target_type_name {
                fn from(e: #error_type_name) -> Self {
                    return #target_type_name::#variant_constructor_name(e);
                }
            }
        };

        return TokenStream::from(from);
    });

    return std::iter::FromIterator::from_iter(streams);
}

fn splice_constructors(new_ctrs: &Vec<(syn::Path, syn::Variant)>, declenum: &DeclEnum) -> TokenStream {
    let mut vs = declenum.data.variants.clone();
    vs.extend(new_ctrs.iter().map(|(_,v)| v.clone()));

    let new_enum = syn::DeriveInput {
        attrs: declenum.attrs.clone(),
        vis: declenum.vis.clone(),
        ident: declenum.ident.clone(),
        generics: declenum.generics.clone(),
        data: syn::Data::Enum(syn::DataEnum { variants: vs, ..declenum.data })
    };

    TokenStream::from(quote! { #new_enum })
}

fn build_constructors(args: &ComposeErrorsAttribute, declenum: &DeclEnum) -> Vec<(syn::Path,syn::Variant)> {
    let prefix = match &args.prefix {
        PrefixOptions::TypeNamePrefix => declenum.ident.to_string(),
        PrefixOptions::CustomPrefix(p) => p.clone(),
    };

    let ctrs: Vec<(syn::Path, syn::Variant)> = args
        .error_types
        .iter()
        .map(|e| {
            let ctr_name: String = format!("{}_{}", prefix, sanitize_type_name(&e));

            let ty = syn::Type::Path(syn::TypePath {
                qself: None,
                path: e.clone(),
            });
            let field = syn::Field {
                attrs: vec![],
                vis: syn::Visibility::Inherited,
                ident: None,
                colon_token: None,
                ty: ty,
            };

            let mut punc = syn::punctuated::Punctuated::new();
            punc.push_value(field);

            let variant = syn::Variant {
                attrs: vec![],
                ident: syn::Ident::new(&&ctr_name, Span::call_site()),
                fields: syn::Fields::Unnamed(syn::FieldsUnnamed {
                    paren_token: syn::token::Paren {
                        span: Span::call_site(),
                    },
                    unnamed: punc,
                }),
                discriminant: None,
            };

            return (e.clone(), variant);
        })
        .collect();

    return ctrs;
}

fn sanitize_type_name(path: &syn::Path) -> String {
    //TODO: Clean this up.
    let mut st = String::new();
    return path.segments.iter().fold(st, |mut acc, curr| {
        acc.push_str("_");
        acc.push_str(&curr.ident.to_string());
        return acc;
    });
}
