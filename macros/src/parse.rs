use proc_macro2::Ident;
use syn::{Type, parse::{Parse, ParseStream}, Token, spanned::Spanned, Path, Visibility};


#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Chain {
    pub name: Ident,
    pub fields: Vec<ChainField>
}

impl Parse for Chain {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;

        input.parse::<syn::Token![=>]>()?;

        let braced_input;
        syn::braced!(braced_input in input);

        let fields = {
            let mut fields = Vec::new();

            while !braced_input.is_empty() {
                fields.push(braced_input.parse::<ChainField>()?);

                let lookahead = braced_input.lookahead1();

                if lookahead.peek(Token![;]) {
                    braced_input.parse::<Token![;]>()?;
                }
            }

            fields
        };

        Ok(Chain {
            name,
            fields
        })
    }
}

#[derive(Clone)]
pub struct ChainField {
    pub parents: Vec<Path>,
    pub field: ChainFieldData
}

impl Parse for ChainField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut lookahead = input.lookahead1();

        let mut parents = Vec::new();

        while lookahead.peek(Token![@]) {
            input.parse::<Token![@]>()?;

            parents.push(input.parse::<Path>()?);

            lookahead = input.lookahead1();
        }

        let field = input.parse::<ChainFieldData>()?;

        Ok(ChainField {
            parents,
            field
        })
    }
}

#[derive(Clone)]
pub enum ChainFieldData {
    Const(Visibility, Ident, Type)
}

impl Parse for ChainFieldData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vis = input.parse::<Visibility>()?;

        let lookahead = input.lookahead1();

        if lookahead.peek(Token![const]) {
            input.parse::<Token![const]>()?;

            let name = input.parse::<Ident>()?;

            input.parse::<Token![:]>()?;

            let ty = input.parse::<Type>()?;

            Ok(ChainFieldData::Const(vis, name, ty))
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Clone)]
pub struct UseChains(pub Vec<Path>);

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

#[derive(Clone)]
pub struct ImplChains {
    pub ty: syn::Type,
    pub impls: Vec<ImplChain>
}

impl syn::parse::Parse for ImplChains {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ty = input.parse::<syn::Type>()?;

        input.parse::<syn::Token![=>]>()?;

        let braced_input;
        syn::braced!(braced_input in input);

        let mut impls = Vec::new();

        while !braced_input.is_empty() {
            let func = braced_input.parse::<syn::TraitItemFn>()?;

            braced_input.parse::<syn::Token![in]>()?;

            let chain = braced_input.parse::<syn::Path>()?;

            impls.push(ImplChain {
                func,
                chain
            });

            if !braced_input.is_empty() {
                braced_input.parse::<syn::Token![;]>()?;
            }
        }

        Ok(ImplChains {
            ty,
            impls
        })
    }
}

#[derive(Clone)]
pub struct ImplChain {
    pub func: syn::TraitItemFn,
    pub chain: syn::Path
}