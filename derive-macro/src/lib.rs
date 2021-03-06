#[macro_use]
extern crate quote;
extern crate syn;

macro_rules! my_quote {
    ($($t:tt)*) => (quote_spanned!(proc_macro2::Span::call_site() => $($t)*))
}

use proc_macro::{self, TokenStream};
use proc_macro2::{TokenStream as TokenStream2, Ident};
use syn::{Token};
use regex::Regex;
use std::prelude::v1::Vec;

#[proc_macro_derive(Endpoint, attributes(method, endpoint, query))]
pub fn endpoint(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("Couldn't parse item");
    let result = match ast.data {
        syn::Data::Struct(ref s) => endpoint_for_struct(&ast, &s.fields),
        _ => panic!("sorry, Show is not implemented for union or enum type.")
    };

    result.into()
}

fn endpoint_for_struct(
    ast: &syn::DeriveInput,
    fields: &syn::Fields,
) -> TokenStream {
    match *fields {
        syn::Fields::Named(ref fields) => endpoint_impl(&ast, &fields.named),
        _ => panic!("sorry, may it's a complicated struct.")
    }
}


enum FieldAttr {
    Value(proc_macro2::TokenStream),
    OptionValue(proc_macro2::TokenStream),
    None,
}

impl FieldAttr {
    pub fn as_tokens(&self) -> proc_macro2::TokenStream {
        match *self {
            FieldAttr::Value(ref s) | FieldAttr::OptionValue(ref s) => my_quote!(#s),
            FieldAttr::None => my_quote!(None),
        }
    }

    pub fn parse(attrs: &[syn::Attribute]) -> Option<FieldAttr> {
        use syn::{AttrStyle, Meta, NestedMeta};
        let mut result = None;
        let list = ["method", "endpoint", "query"];

        for attr in attrs.iter() {
            match attr.style {
                AttrStyle::Outer => {}
                _ => continue,
            }

            let last_attr_path = attr
                .path
                .segments
                .iter()
                .last()
                .expect("Expected at least one segment where #[segment[::segment*](..)]");

            if !list.contains(&(*last_attr_path).ident.to_string().as_str()) {
                continue;
            }

            let meta = match attr.parse_meta() {
                Ok(meta) => meta,
                Err(_) => continue
            };

            let list = match meta {
                Meta::List(l) => l,
                _ if meta.path().is_ident("method") => {
                    panic!("Invalid #[method] attribute, expected #[method(..)]")
                }
                _ if meta.path().is_ident("endpoint") => {
                    panic!("Invalid #[endpoint] attribute, expected #[endpoint(..)]")
                }
                _ if meta.path().is_ident("query") => {
                    panic!("Invalid #[query] attribute, expected #[query(..)]")
                }
                _ => continue,
            };

            for item in list.nested.iter() {
                match *item {
                    NestedMeta::Meta(Meta::Path(ref path)) => {
                        if (*last_attr_path).ident.to_string().as_str() == "method" {
                            let tokens = match path.get_ident().unwrap().to_string().to_uppercase().as_str() {
                                "GET" | "PUT" | "POST" | "DELETE" => {
                                    path.get_ident().cloned().unwrap().to_string().to_uppercase()
                                }
                                _ => panic!("Invalid #[method] attribute, expected #[method([GET|PUT|POST|DELETE])]")
                            };
                            result = Some(FieldAttr::Value(my_quote!(#tokens)));
                        } else if (*last_attr_path).ident.to_string().as_str() == "query" {
                            if path.get_ident().unwrap().to_string().as_str() == "None" {
                                result = Some(FieldAttr::None);
                            } else {
                                let tokens = path.get_ident().cloned().unwrap().to_string();
                                result = Some(FieldAttr::OptionValue(my_quote!(Some(#tokens))));
                            }
                        }
                    }
                    NestedMeta::Lit(syn::Lit::Str(ref s)) => {
                        let tokens = s.clone().value();
                        if (*last_attr_path).ident.to_string().as_str() == "query" {
                            result = Some(FieldAttr::OptionValue(my_quote!(Some(#tokens))));
                        } else {
                            result = Some(FieldAttr::Value(my_quote!(#tokens)));
                        }
                    }
                    NestedMeta::Lit(syn::Lit::Int(ref s)) => {
                        let tokens = s;
                        if (*last_attr_path).ident.to_string().as_str() == "query" {
                            result = Some(FieldAttr::OptionValue(my_quote!(Some(#tokens))));
                        } else {
                            result = Some(FieldAttr::Value(my_quote!(#tokens)));
                        }
                    }
                    _ => panic!("Invalid #[method([GET|PUT|POST|DELETE])] or #[endpoint(\"...\")] attribute")
                }
            }
        }

        result
    }
}


struct FieldExt<'a> {
    ty: &'a syn::Type,
    attr: Option<FieldAttr>,
    ident: syn::Ident,
}

impl<'a> FieldExt<'a> {
    pub fn new(field: &'a syn::Field) -> FieldExt<'a> {
        FieldExt {
            ty: &field.ty,
            attr: FieldAttr::parse(&field.attrs),
            ident: field.ident.clone().unwrap(),
        }
    }

    pub fn as_arg(&self) -> TokenStream2 {
        let f_name = &self.ident;
        let ty = &self.ty;
        my_quote!(#f_name: #ty)
    }

    pub fn as_init(&self) -> TokenStream2 {
        let f_name = &self.ident;
        let init = match self.attr {
            None => my_quote!(#f_name),
            Some(ref attr) => attr.as_tokens(),
        };

        my_quote!(#f_name: #init)
    }

    pub fn as_endpoint(&self) -> Vec<TokenStream2> {
        let f_name = &self.ident;
        let e = match self.attr {
            None => my_quote!(#f_name).to_string(),
            Some(ref attr) => attr.as_tokens().to_string(),
        };
        let re = Regex::new(r"\{(?P<ident>\w+)}").unwrap();
        let iter = re.find_iter(e.as_str());
        let mut endpoint_init: Vec<TokenStream2> = Vec::new();

        for m in iter {
            let cap = re.captures(m.as_str()).unwrap();
            let value = cap.name("ident").unwrap();
            let bv = Ident::new(value.as_str(), proc_macro2::Span::call_site());
            let b = m.as_str();

            endpoint_init.push(my_quote! {
                #b => after = after.replace(#b, self.#bv.to_string().as_str())
            })
        }

        endpoint_init
    }

    pub fn as_query(&self) -> TokenStream2 {
        let bv = Ident::new(&self.ident.to_string(), proc_macro2::Span::call_site());
        let b = self.ident.to_string();
        my_quote! {
               #b => {
                    if self.#bv.is_some() {
                         query.push_str(format!("{}={:?}",
                                               k = #b,
                                               v = self.#bv.as_ref()
                         ).as_str());
                        query.push_str("&");
                    }
                }
            }
    }

    pub fn is_endpoint(&self) -> bool {
        let f_name = &self.ident;

        f_name.to_string() == "endpoint"
    }

    pub fn is_query(&self) -> bool {
        if self.has_attr() {
            match self.attr.as_ref().unwrap() {
                FieldAttr::OptionValue(_) | FieldAttr::None => true,
                _ => false
            }
        } else {
            return false;
        }
    }

    pub fn needs_arg(&self) -> bool {
        !self.has_attr()
    }

    pub fn has_attr(&self) -> bool {
        self.attr.is_some()
    }
}

fn endpoint_impl(
    ast: &syn::DeriveInput,
    fields: &syn::punctuated::Punctuated<syn::Field, Token![,]>,
) -> TokenStream {
    let name = &ast.ident;
    let fields: Vec<FieldExt> = fields
        .iter()
        .enumerate()
        .map(|(_, f)| FieldExt::new(f))
        .collect();
    let args = fields.iter().filter(|f| f.needs_arg()).map(|f| f.as_arg());
    let inits = fields.iter().map(|f| f.as_init());
    let query_fields: Vec<TokenStream2> = fields.iter().filter(|f| f.is_query()).map(|f| {
        let name = f.ident.to_string();
        my_quote!(#name)
    }).collect();
    let mut query_str_token = my_quote!(#(#query_fields),*);
    if query_str_token.to_string().as_str() == "[]" {
        query_str_token = my_quote!(vec![""]);
    } else {
        query_str_token = my_quote!(vec![#query_str_token]);
    }

    let mut ma = my_quote!();
    for field in fields.iter() {
        if field.is_endpoint() {
            let value = field.as_endpoint();
            ma = my_quote!(#(#value),*);
        }
    }

    let mut query_vec: Vec<TokenStream2> = Vec::new();
    for field in fields.iter() {
        if field.is_query() {
            let value = field.as_query();
            query_vec.push(value);
        }
    }
    let mut mb = my_quote!(#(#query_vec),*);

    if mb.is_empty() {
        mb = my_quote!(
            _ => panic!("no"),
        )
    } else {
        mb = my_quote!(
            #mb,
            _ => panic!("no"),
        )
    }

    for field in fields.iter() {
        if field.is_query() {
        }
    }

    let inits = my_quote!({ #(#inits),* });
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let new = syn::Ident::new("new", proc_macro2::Span::call_site());
    let doc = format!("Constructs a new `{}`.", name);
    let get_endpoint = syn::Ident::new("get_endpoint", proc_macro2::Span::call_site());
    let get_endpoint_doc = format!("get the endpoint for gitlab `{}()`.", get_endpoint);
    let get_query_fields = syn::Ident::new("get_query_fields", proc_macro2::Span::call_site());
    let get_query = syn::Ident::new("get_query", proc_macro2::Span::call_site());

    let output = my_quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #[doc = #doc]
            pub fn #new(#(#args),*) -> Self {
                #name #inits
            }
        }

        impl #impl_generics EndPointTrait for #name #ty_generics {
             #[doc = #get_endpoint_doc]
             fn #get_endpoint(&self) -> String {
                let re = regex::Regex::new(r"\{\w+}").unwrap();
                let iter = re.find_iter(self.endpoint);
                let mut after = self.endpoint.to_string();
                for m in iter {
                   match m.as_str() {
                       #ma,
                       _ => panic!("error")
                    }
                }
                after
             }

             fn #get_query(&self) -> String {
                let mut query = String::new();
                query.push_str("?");
                for name in self.#get_query_fields() {
                    match name {
                        #mb
                    }
                }
                query.pop();
                query
             }

             fn #get_query_fields(&self) -> Vec<&'static str> {
                #query_str_token
             }

             fn get_method(&self) -> Kind {
                match self.method {
                    "GET" => Kind::GET,
                    "PUT" => Kind::PUT,
                    "POST" => Kind::POST,
                    "DELETE" => Kind::DELETE,
                    _ => panic!("not support method type, just support[GET|PUT|POST|DELETE]")
                }
            }
        }
    };

    output.into()
}
