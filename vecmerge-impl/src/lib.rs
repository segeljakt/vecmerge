extern crate proc_macro;

use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub fn vecmerge(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expr = syn::parse_macro_input!(input as syn::Expr);
    let mut elems = Vec::new();
    extract(expr, &mut elems);
    let result = proc_quote::quote! {{
        let mut v = Vec::new();
        #(#elems)*
        v
    }};
    result.into()
}

fn extract(expr: syn::Expr, elems: &mut Vec<proc_macro2::TokenStream>) {
    match expr {
        syn::Expr::Binary(expr) => {
            if let syn::BinOp::Add(..) = expr.op {
                extract(*expr.left, elems);
                extract(*expr.right, elems);
            }
        }
        syn::Expr::Array(array) => {
            for elem in array.elems.into_iter() {
                elems.push(proc_quote::quote!(v.push(#elem);))
            }
        }
        vector => {
            elems.push(proc_quote::quote!(v.extend(#vector);));
        }
    }
}
