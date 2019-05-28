extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_hack::proc_macro_hack;
use proc_quote::quote;

#[proc_macro_hack]
pub fn vecmerge(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expr = syn::parse_macro_input!(input as syn::Expr);
    let mut visitor = Visitor::new();
    visitor.visit(&expr);
    let (capacity, parts) = visitor.get();
    proc_macro::TokenStream::from(quote!({
        let mut vec = Vec::with_capacity(#(#capacity)+*);
        #(#parts)*
        vec
    }))
}

struct Visitor {
    capacity: Vec<TokenStream>,
    parts: Vec<TokenStream>,
}

impl Visitor {
    fn new() -> Self {
        Self {
            capacity: Vec::new(),
            parts: Vec::new(),
        }
    }

    fn get(self) -> (Vec<TokenStream>, Vec<TokenStream>) {
        (self.capacity, self.parts)
    }

    fn visit(&mut self, expr: &syn::Expr) {
        match expr {
            syn::Expr::Binary(ref binary_expr) => match binary_expr.op {
                syn::BinOp::Add(..) => {
                    self.visit(binary_expr.left.as_ref());
                    self.visit(binary_expr.right.as_ref());
                }
                op => self.parts.push(quote! {
                    compile_error!(format!("Unexpected operator {}", #op))
                }),
            },
            syn::Expr::Try(ref try_expr) => match try_expr.expr.as_ref() {
                syn::Expr::Array(array_expr) => self.visit_try_array_expr(array_expr),
                _ => self.visit_expr(expr),
            },
            syn::Expr::Array(array_expr) => self.visit_array_expr(array_expr),
            _ => self.visit_expr(expr),
        }
    }

    fn visit_array_expr(&mut self, array_expr: &syn::ExprArray) {
        let len = array_expr.elems.len();
        self.capacity.push(quote!(#len));
        for elem in array_expr.elems.iter() {
            self.parts.push(quote! {
                vec.push(#elem);
            });
        }
    }

    fn visit_try_array_expr(&mut self, try_array_expr: &syn::ExprArray) {
        let elems = &try_array_expr.elems;
        let names = (0..elems.len())
            .into_iter()
            .map(|index| Ident::new(&format!("elem_{}", index), Span::call_site()))
            .collect::<Vec<Ident>>();
        let pattern = quote!((#(Some(#names)),*));
        let expr = quote!((#elems));
        let pushes = quote!(#(vec.push(#names);)*);
        self.parts.push(quote! {
            if let #pattern = #expr {
                #pushes
            }
        });
    }

    fn visit_expr(&mut self, expr: &syn::Expr) {
        self.capacity.push(quote!(#expr.len()));
        self.parts.push(quote! { vec.extend(#expr); });
    }
}
