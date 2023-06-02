use proc_macro2::Ident;
use syn::{Type, parse::{Parse, ParseStream}, Token, spanned::Spanned, Path, Visibility, Generics};


#[derive(Clone)]
pub struct Chainlink {
    pub name: Ident,
    pub generics: Vec<Type>,
    pub fields: Vec<ChainlinkField>,
}

impl Parse for Chainlink {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;

        let lookahead = input.lookahead1();
        let generics = if lookahead.peek(Token![<]) {
            let mut generics = Vec::new();

            input.parse::<Token![<]>()?;

            while !input.peek(Token![>]) {
                generics.push(input.parse::<Type>()?);

                let lookahead = input.lookahead1();

                if lookahead.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                }
            }

            input.parse::<Token![>]>()?;

            generics
        } else {vec![]};

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
            generics,
            fields
        })
    }
}

#[derive(Clone)]
pub enum ChainlinkField {
    Const(Ident, Type),
    Mut(Ident, Type),
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
        } else if lookahead.peek(Token![mut]) {
            input.parse::<Token![mut]>()?;

            let name = input.parse::<Ident>()?;

            input.parse::<Token![:]>()?;

            let ty = input.parse::<Type>()?;

            Ok(ChainlinkField::Mut(name, ty))
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
    pub generics: Vec<Type>,
    pub fields: Vec<ChainField>
}

impl Parse for Chain {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;

        let lookahead = input.lookahead1();
        let generics = if lookahead.peek(Token![<]) {
            let mut generics = Vec::new();

            input.parse::<Token![<]>()?;

            while !input.peek(Token![>]) {
                generics.push(input.parse::<Type>()?);

                let lookahead = input.lookahead1();

                if lookahead.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                }
            }

            input.parse::<Token![>]>()?;

            generics
        } else {vec![]};

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
            generics,
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
    Const(Visibility, Ident, Type),
    Mut(Ident, Type)
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
        } else if lookahead.peek(Token![mut]) {
            if vis != Visibility::Inherited {
                return Err(syn::Error::new(vis.span(), "Chainlink fields must be of inherited visibility"));
            }

            input.parse::<Token![mut]>()?;

            let name = input.parse::<Ident>()?;

            input.parse::<Token![:]>()?;

            let ty = input.parse::<Type>()?;

            Ok(ChainFieldData::Mut(name, ty))
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
    pub impls: Vec<ImplChain>,
    pub where_clause: Option<Generics>
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

            let lookahead = braced_input.lookahead1();

            if lookahead.peek(Token![;]) {
                braced_input.parse::<Token![;]>()?;
            }
        }

        let lookahead = input.lookahead1();

        let where_clause = if lookahead.peek(Token![where]) {
            input.parse::<Token![where]>()?;
            Some(input.parse::<Generics>()?)
        } else {
            None
        };

        Ok(ImplChains {
            ty,
            impls,
            where_clause
        })
    }
}

#[derive(Clone)]
pub struct ImplChain {
    pub func: syn::TraitItemFn,
    pub chain: syn::Path
}