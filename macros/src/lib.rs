#![deny(missing_docs)]

//! # `typechain-macros`
//! 
//! This crate contains macros for working with
//! related type functionality. Using dynamic
//! dispatch, it is possible to create a chain
//! of traits that can be used to access the
//! fields of a struct.
//! 
//! The macros in this crate use user-defined traits
//! and structs to generate an easy-to-use chain. See
//! the [`typechain`](https://crates.io/crates/typechain)
//! crate for more information.

extern crate proc_macro;

use std::collections::{HashMap, hash_map::Entry};

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{proc_macro_error, abort_call_site};
use quote::quote;
use syn::{ItemTrait, TypeParamBound, Fields, Meta, Path, spanned::Spanned};


/// Create a chainlink trait.
/// 
/// The trait will be renamed to `{{name}}Chainlink`,
/// and the original name will be used for the
/// associated type (dyn `{{name}}Chainlink`).
#[proc_macro_error]
#[proc_macro_attribute]
pub fn chainlink(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast: ItemTrait = match syn::parse(input) {
        Ok(ast) => ast,
        Err(_) => abort_call_site!("Chainlink derivations can only be applied to traits")
    };

    let name = ast.ident.clone();
    let visibility = ast.vis.clone();
    let name_str = name.to_string();

    let new_name = syn::Ident::new(&format!("{}Chainlink", name_str), Span::call_site());

    ast.ident = new_name.clone();

    ast.supertraits.iter_mut().for_each(|s| {
        if let TypeParamBound::Trait(t) = s {
            let name = t.path.segments.last().unwrap().ident.clone();
            
            t.path.segments.last_mut().unwrap().ident = syn::Ident::new(&format!("{}Chainlink", name), Span::call_site());
        }
    });

    let expanded = quote! {
        #ast

        #visibility type #name = dyn #new_name;
    };

    expanded.into()
}

/// Derive chains for a struct.
/// 
/// This macro will generate chain implementations
/// for the traits specified in the `chain` attribute.
/// Any annotated fields will generate a getter method
/// for that chainlink trait.
#[proc_macro_error]
#[proc_macro_derive(Chain, attributes(chain))]
pub fn chain_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = ast.ident.clone();
    let ast = match ast.data {
        syn::Data::Struct(s) => s,
        _ => abort_call_site!("Chain can only be derived for structs")
    };

    if let Fields::Named(_) = ast.fields {} else {
        abort_call_site!("Chain can only be derived for structs with named fields")
    }

    let impls = ast.fields.iter().filter(|f| {
        f.attrs.iter().any(|a| {
            a.path().is_ident("chain")
        })
    }).flat_map(|f| {
        let name = f.ident.clone().unwrap();
        let ty = f.ty.clone();

        f.attrs.iter().filter(|a| {
            a.path().is_ident("chain")
        }).map(move |a| {
            let meta = match a.parse_args::<Meta>() {
                Ok(meta) => meta,
                Err(_) => abort_call_site!("Invalid chain attribute")
            };

            let mut path = match meta {
                Meta::Path(p) => p,
                _ => abort_call_site!("Invalid chain attribute")
            };

            path.segments.last_mut().unwrap().ident = syn::Ident::new(&format!("{}Chainlink", path.segments.last().unwrap().ident), Span::call_site());

            (path, name.clone(), ty.clone())
        })
    }).collect::<Vec<_>>();

    let mut traits: HashMap<Path, Vec<proc_macro2::TokenStream>> = HashMap::new();

    for (trait_, attr_name, attr_ty) in impls {
        let tokens = quote! {
            fn #attr_name(&self) -> #attr_ty {
                self.#attr_name.clone()
            }
        };

        if let Entry::Vacant(_) = traits.entry(trait_.clone()) {
            traits.insert(trait_.clone(), vec![]);
        }

        traits.get_mut(&trait_).unwrap().push(tokens);
    }

    let traits = traits.iter().map(|(trait_, tokens)| {
        let trait_ = trait_.clone();
        let tokens = tokens.clone();

        quote! {
            impl #trait_ for #name {
                #(#tokens)*
            }
        }
    }).collect::<Vec<_>>();

    let expanded = quote! {
        #(#traits)*
    };

    expanded.into()
}

/// Import chainlink traits.
/// 
/// This is a helper macro for importing chainlink
/// traits and their associated types.
#[proc_macro_error]
#[proc_macro]
pub fn use_chains(input: TokenStream) -> TokenStream {
    let paths = syn::parse_macro_input!(input as UseChains);

    let paths = paths.0.iter().map(|p| {
        let mut path = p.clone();
        path.segments.last_mut().unwrap().ident = syn::Ident::new(&format!("{}Chainlink", path.segments.last().unwrap().ident), p.span());

        quote! {
            use #path;
            use #p;
        }
    }).collect::<Vec<_>>();

    let expanded = quote! {
        #(#paths)*
    };

    expanded.into()
}

struct UseChains(Vec<Path>);

impl syn::parse::Parse for UseChains {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut paths = Vec::new();

        while !input.is_empty() {
            paths.push(input.parse::<Path>()?);

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(UseChains(paths))
    }
}