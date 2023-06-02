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

use parse::{ChainlinkField, ChainFieldData};
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{proc_macro_error, emit_error, abort_if_dirty};
use quote::{quote, ToTokens, quote_spanned};
use syn::{Path, spanned::Spanned, Visibility};

mod parse;


/// Create a chainlink trait.
/// 
/// The trait will be renamed to `{{name}}Chainlink`,
/// and the original name will be used for the
/// associated type (dyn `{{name}}Chainlink`).
#[proc_macro_error]
#[proc_macro]
pub fn chainlink(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as parse::Chainlink);

    let name = ast.name.clone();
    let generics = ast.generics.clone();
    let generics = quote! {
        < #( #generics ),* >
    };

    let fields = ast.fields.iter().map(|f| {
        match f {
            ChainlinkField::Const(name, ty) => {
                quote! {
                    fn #name(&self) -> & #ty;
                }
            },
            ChainlinkField::Mut(name, ty) => {
                quote! {
                    fn #name(&mut self) -> &mut #ty;
                }
            },
            ChainlinkField::Static(name, ty) => {
                quote! {
                    fn #name() -> &'static #ty;
                }
            },
            ChainlinkField::Fn(func) => {
                let name = func.sig.ident.clone();
                let generics = func.sig.generics.clone();
                let inputs = func.sig.inputs.clone();
                let output = func.sig.output.clone();
                let where_clause = func.sig.generics.where_clause.clone();

                quote! {
                    #generics
                    fn #name(#inputs) #output #where_clause;
                }
            }
        }
    });

    let trait_name = syn::Ident::new(&format!("{}Chainlink", name), Span::call_site());

    let expanded = quote! {
        #[allow(missing_docs)]
        pub trait #trait_name #generics {
            #(#fields)*
        }

        #[allow(missing_docs)]
        pub type #name #generics = dyn #trait_name #generics;
    };

    expanded.into()
}

/// Create a chain.
#[proc_macro_error]
#[proc_macro]
pub fn chain(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as parse::Chain);

    let name = ast.name.clone();
    let generics = ast.generics.clone();

    let generics = quote! {
        < #( #generics ),* >
    };

    let fields = ast.fields.iter().filter_map(|f| {
        match f.field.clone() {
            ChainFieldData::Const(vis, name, ty) => {
                Some(quote! {
                    #vis #name: #ty
                })
            },
            ChainFieldData::Mut(name, ty) => {
                Some(quote! {
                    #name: #ty
                })
            },
            ChainFieldData::Static(..) => {
                None
            }
        }
    });

    let trait_funcs: HashMap<Path, Vec<proc_macro2::TokenStream>> = ast.fields.iter().fold(HashMap::new(), |mut map, f| {
        let parents = f.parents.clone();

        for parent in parents {
            if let Entry::Vacant(_) = map.entry(parent.clone()) {
                map.insert(parent.clone(), vec![]);
            }

            let tokens = match f.field.clone() {
                ChainFieldData::Const(vis, name, ty) => {
                    if !matches!(vis, Visibility::Inherited) {
                        emit_error!(vis, "Chainlink fields must be of inherited visibility");
                    }

                    quote! {
                        fn #name(&self) -> & #ty {
                            &self.#name
                        }
                    }
                },
                ChainFieldData::Mut(name, ty) => {
                    quote! {
                        fn #name(&mut self) -> &mut #ty {
                            &mut self.#name
                        }
                    }
                },
                ChainFieldData::Static(name, ty, expr) => {
                    quote_spanned! { expr.span() =>
                        fn #name() -> &'static #ty {
                            &#expr
                        }
                    }
                }
            };

            abort_if_dirty();

            map.get_mut(&parent).unwrap().push(tokens);
        }

        map
    });

    let trait_impls = trait_funcs.iter().map(|(trait_, tokens)| {
        let mut trait_ = trait_.clone();
        trait_.segments.last_mut().unwrap().ident = syn::Ident::new(&format!("{}Chainlink", trait_.segments.last().unwrap().ident), trait_.span());

        let tokens = tokens.clone();

        quote! {
            impl #generics #trait_ for #name #generics {
                #(#tokens)*
            }
        }
    });

    let expanded = quote! {
        pub struct #name #generics {
            #(#fields),*
        }

        #(#trait_impls)*
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
    let paths = syn::parse_macro_input!(input as parse::UseChains);

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

/// Manually implement chains.
/// 
/// This macro will generate chain implementations
/// manually. This is useful when you want to implement
/// chains for a type that you don't own.
#[proc_macro_error]
#[proc_macro]
pub fn impl_chains(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as parse::ImplChains);

    let ty = ast.ty.clone();
    let where_clause = ast.where_clause.clone();

    let mut impls: HashMap<Path, Vec<proc_macro2::TokenStream>> = HashMap::new();

    for impl_ in ast.impls {
        let tokens = impl_.func.to_token_stream();

        if let Entry::Vacant(_) = impls.entry(impl_.chain.clone()) {
            impls.insert(impl_.chain.clone(), vec![]);
        }

        impls.get_mut(&impl_.chain).unwrap().push(tokens);
    }

    let impls = impls.iter().map(|(trait_, tokens)| {
        let mut trait_ = trait_.clone();
        trait_.segments.last_mut().unwrap().ident = syn::Ident::new(&format!("{}Chainlink", trait_.segments.last().unwrap().ident), trait_.span());

        let tokens = tokens.clone();

        quote! {
            impl #where_clause #trait_ for #ty {
                #(#tokens)*
            }
        }
    }).collect::<Vec<_>>();

    let expanded = quote! {
        #(#impls)*
    };

    expanded.into()
}