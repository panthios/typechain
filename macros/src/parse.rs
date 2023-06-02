use proc_macro2::Ident;
use syn::{Type, parse::{Parse, ParseStream}, Token, spanned::Spanned};


pub struct Chainlink {
    pub name: Ident,
    pub fields: Vec<ChainlinkField>,
}

impl Parse for Chainlink {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;

        input.parse::<syn::Token![=>]>()?;

        let braced_input;
        syn::braced!(braced_input in input);

        let fields = {
            let mut fields = Vec::new();

            while !braced_input.is_empty() {
                fields.push(braced_input.parse::<ChainlinkField>()?);

                let lookahead = braced_input.lookahead1();

                if lookahead.peek(Token![;]) {
                    braced_input.parse::<Token![;]>()?;
                }
            }

            fields
        };

        Ok(Chainlink {
            name,
            fields
        })
    }
}

pub enum ChainlinkField {
    Const(Ident, Type),
    Fn(syn::TraitItemFn)
}

impl Parse for ChainlinkField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![const]) {
            input.parse::<Token![const]>()?;

            let name = input.parse::<Ident>()?;

            input.parse::<Token![:]>()?;

            let ty = input.parse::<Type>()?;

            Ok(ChainlinkField::Const(name, ty))
        } else if lookahead.peek(Token![fn]) {
            let func = input.parse::<syn::TraitItemFn>()?;

            if func.default.is_some() {
                return Err(syn::Error::new(func.default.unwrap().span(), "Chains cannot have default functions"));
            }

            Ok(ChainlinkField::Fn(func))
        } else {
            Err(lookahead.error())
        }
    }
}