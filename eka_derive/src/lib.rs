use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::{format_ident};

trait Idable {
    const MAX: usize = 0;
    fn idx(&self) -> usize;
}



#[proc_macro_derive(Idable)]
pub fn derive_idable(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_idable(&ast)
}


fn impl_idable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut variant_names: Vec::<syn::Ident>  = vec!();
    let mut variant_names_field : Vec::<syn::Ident>  = vec!();
    let mut variant_names_named_field : Vec::<syn::Ident>  = vec!();


    let variants = if let  syn::Data::Enum(data) = &ast.data {
        data.variants.iter()
    } else {
        panic!("Idable cannot be derived on non-enum");
    };

    let variant_count = variants.len();
    for variant in variants {
        match &variant.fields {
            syn::Fields::Named(f) => {
                for p in f.named.clone().into_pairs() {
                    let f: syn::Field = p.value().clone();
                    if matches!(f.ty, syn::Type::Verbatim(_)) {
                            println!("{}", f.ty.to_token_stream());
                    } if let syn::Type::Path(p) = f.ty {

                            println!("{:?}", p.path.get_ident().unwrap());
                    } 
                    //println!("{}", f.ty.to_token_stream());

                    //if has_bound(f.ty(), proc_macro::Ident::new()) {

                    //} 
                }
                variant_names_named_field.push(variant.ident.clone())
            },
            syn::Fields::Unnamed(_) => variant_names_field.push(variant.ident.clone()),
            syn::Fields::Unit => variant_names.push(variant.ident.clone())
        }
    }


    let mut indexes=vec!();
    let mut indexes_field=vec!();
    let mut indexes_named_field=vec!();
    for i in 0..variant_names.len() {
        indexes.push(i);
    }
    let end = variant_names.len() + variant_names_field.len();
    for i in variant_names.len()..end {
        indexes_field.push(i);
    }
    for i in end..(end + variant_names_named_field.len()) {
        indexes_named_field.push(i);
    }


    let gen = quote! {
        impl Idable for #name {
            const MAX: usize = #variant_count;
            fn idx(&self) -> usize {
                match self {
                    #(Self::#variant_names => #indexes,)* 
                    #(Self::#variant_names_field(..) => #indexes_field,)* 
                    #(Self::#variant_names_named_field{..} => #indexes_named_field,)* 
                }
            }
        }
    };
    print!("{}", gen);
    gen.into()
}

fn format_match_arm(ident: syn::Ident) -> syn::Arm {
    let pat = syn::PatIdent {
        attrs: vec!(),
        by_ref: None,
        mutability: None,
        ident: ident,
        subpat: None,
    };

    syn::Arm {
        attrs: vec!(),
        pat: syn::Pat::Ident(pat),
        guard: None,
        fat_arrow_token: <syn::Token![=>]>::default(),
        body: Box::new(format_expression()),
        comma: Some(<syn::Token![,]>::default()),
    }
}

fn format_expression() -> syn::Expr {

    let mut statements = vec!();
    let statement = make_statements();
    statements.push(statement);
    let block = syn::Expr::Block(
        syn::ExprBlock {
            attrs: vec!(),
            label: None,
            block: syn::Block {
                stmts: statements,
                brace_token: Default::default()
            },
        }
    );
    block
}

fn make_statements() -> syn::Stmt {
    syn::Stmt::Local(
        syn::Local {
            attrs: vec!(),
            let_token: <syn::Token![let]>::default(),
            pat: 
                syn::Pat::Ident( 
                    syn::PatIdent{
                    attrs: vec!(), 
                    by_ref: None, 
                    mutability:  None, 
                    ident: format_ident!("v1"), 
                    subpat: None
                }),
            init: Some(
                syn::LocalInit {
                    eq_token: <syn::Token![=]>::default(),
                    expr: Box::new(
                        syn::Expr::Binary(
                            syn::ExprBinary {
                                attrs: vec!(),
                                left: Box::new(syn::Expr::Lit(
                                    syn::ExprLit {
                                         attrs: vec!(), 
                                         lit: syn::Lit::Int(syn::LitInt::new("2", proc_macro2::Span::call_site() )),
                                    })),
                                right:Box::new(syn::Expr::Lit(
                                    syn::ExprLit {
                                         attrs: vec!(), 
                                         lit: syn::Lit::Int(syn::LitInt::new("2", proc_macro2::Span::call_site() )),
                                    })),
                                op: syn::BinOp::Add(<syn::Token![+]>::default())
                        })),
                    diverge: None,
            }),
            semi_token: <syn::Token![;]>::default(),
        }
    )
}
//Self::ExampleField3(v1, _, v2, _, _, v3) => {
//#totals  + 
//v3 + v2 * v3.max() + v1 * (v2.max() * v3.max() + v3.max())
//                    } 

