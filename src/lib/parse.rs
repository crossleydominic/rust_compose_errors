use proc_macro2::Span;
use crate::either;
use syn::{Path,Meta, Lit, PathArguments, Error};
use syn::parse::{ParseStream,Result};

#[derive(Debug, Clone)]
pub struct DeclEnum {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub data: syn::DataEnum
}

impl DeclEnum {
    fn parse(input: ParseStream) -> Result<Self> {

        let i: syn::DeriveInput = syn::parse::Parse::parse(input)?;

        match i.data {
            syn::Data::Enum(e) => Result::Ok(DeclEnum {
                attrs: i.attrs,
                vis: i.vis,
                ident: i.ident,
                generics: i.generics,
                data: e
            }),
            _ => Result::Err(Error::new(input.span(), "Composing of errors only valid on Enums"))
        }
    }
}

impl syn::parse::Parse for DeclEnum {
    fn parse(input: ParseStream)-> Result<Self> {
        return DeclEnum::parse(input);
    }
}

#[derive(Debug)]
pub struct ComposeErrorsAttribute {
    pub prefix: PrefixOptions,
    pub error_types: Vec<Path>,
}

impl ComposeErrorsAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed_args : Vec<Meta> = parse_comma_seperated(input)?;

        let attr = ComposeErrorsAttribute::parse_attribute_values(&parsed_args, input.span())?;

        return Result::Ok(attr);

    }

    #[doc(hidden)]
    fn parse_attribute_values(opts: &Vec<Meta>, span: Span) -> Result<ComposeErrorsAttribute>{
        let prefix = ComposeErrorsAttribute::parse_prefix(opts, span)?;
        let errors = ComposeErrorsAttribute::parse_errors(opts, span)?;
        let _      = ComposeErrorsAttribute::ensure_no_lists(opts, span)?; //Don't ignore unsupported contructs

        return Result::Ok(ComposeErrorsAttribute {
            prefix: prefix,
            error_types: errors
        });
    }

    #[doc(hidden)]
    fn parse_prefix(opts: &Vec<Meta>, span: Span) -> Result<PrefixOptions>{
        let (lefts, rights) = either::partition_eithers_moving_all(opts.iter().filter_map(|x|
            match x {
                Meta::NameValue(nv) => Some(nv),
                _ => None
            }).map(|x| {

                let leading_colon = x.path.leading_colon;
                let number_of_segments = x.path.segments.len();
                let first_path = x.path.segments.first();
                let first_path_ident_is_prefix = first_path.map_or(false, |x| x.ident == "prefix");
                let first_path_arg_is_none = first_path.map_or(false, |x| x.arguments == PathArguments::None);
                let prefix_string = match x.lit {
                        Lit::Str(ref st) => st.value(),
                        _ => String::from("")
                };

                if leading_colon == None &&
                    number_of_segments == 1 &&
                    first_path_ident_is_prefix &&
                    first_path_arg_is_none {



                    return either::Either::Right(prefix_string);
                }else{
                    return either::Either::Left(x);
                }
            }));

        if lefts.len() != 0 {
            //TODO: Improve this error message!
            return Result::Err(Error::new(span, "Unknown options"));
        } else if rights.len() > 1 {
            return Result::Err(Error::new(span, "'prefix' options specified more than once"));
        } else if rights.len() == 0 {
            return Result::Ok(PrefixOptions::TypeNamePrefix);
        } else {
            //Only 1 item in the list
            return Result::Ok(PrefixOptions::CustomPrefix(rights.first().unwrap().clone()));
        }
    }

    #[doc(hidden)]
    fn parse_errors(opts: &Vec<Meta>, span: Span) -> Result<Vec<Path>>{
        let paths :Vec<Path> = opts.iter().filter_map(|x|
            match x {
                Meta::Path(p) => Some(p.clone()),
                _ => None
            }).collect();

        if paths.len() == 0 {
            return Result::Err(Error::new(span, "No errors to compose"));
        } else {
            return Result::Ok(paths);
        }
    }

    #[doc(hidden)]
    fn ensure_no_lists(opts: &Vec<Meta>, span: Span) -> Result<()>{
        if opts.iter().all(|x| match x {
            Meta::Path(_) | Meta::NameValue(_) => true,
            Meta::List(_) => false
        }) {
            return Result::Ok(());
        } else {
            return Result::Err(Error::new(span, "arguments of the form `foo(bar,...)` not supported"));
        }
    }
}


impl syn::parse::Parse for ComposeErrorsAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        return ComposeErrorsAttribute::parse(input)
    }
}

#[derive(Debug)]
pub enum PrefixOptions {
    CustomPrefix(String),
    TypeNamePrefix,
}

impl Default for PrefixOptions {
    fn default() -> Self {
        PrefixOptions::TypeNamePrefix
    }
}

//Taken straight out of https://docs.rs/crate/syn/1.0.14/source/src/parse_macro_input.rs.
//I can't the way to invoke a pre-built parser for an AttributeArgs from a ParserStream.
fn parse_comma_seperated<T : syn::parse::Parse>(input: syn::parse::ParseStream) -> syn::Result<Vec<T>> {
    let mut metas = Vec::new();

    loop {
        if input.is_empty() {
            break;
        }
        let value = input.parse()?;
        metas.push(value);
        if input.is_empty() {
            break;
        }
        input.parse::<syn::Token![,]>()?;
    }
    return syn::parse::Result::Ok(metas);
}

