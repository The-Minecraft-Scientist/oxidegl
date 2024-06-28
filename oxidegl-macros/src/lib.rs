use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced, parse::Parse, parse_macro_input, punctuated::Punctuated, token::Enum, File, Ident,
    ItemConst, ItemEnum, Token, Variant,
};

extern crate proc_macro;
#[proc_macro]
/// ### Usage
///
/// ```rust,ignore
/// gl_enum! {
///     PolygonMode {
///         GL_FOO,
///         GL_BAR,
///     }
///     OtherGlThing {
///         GL_BAZ,
///         GL_BAF,
///     }
/// }
/// // Generates the following enum definition (minus absolute path cruft):
///
/// #[repr(u32)]
/// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// pub enum PolygonMode {
///     Foo = GL_FOO,
///     Bar = GL_BAR,
/// }
/// impl From<u32> for PolygonMode {
///     #[cfg(debug_assertions)]
///     fn from(value: u32) -> Self {
///         #[cfg(debug_assertions)]
///         match value {
///             GL_FOO => Self::Foo,
///             GL_BAR => Self::Bar,
///             _ => { panic!("Tried to create PolygonMode from unexpected GLenum {}", value) }
///         }
///         #[cfg(not(debug_assertions))]
///         //Optimized impl for no debug assertions
///         unsafe { transmute(value) }   
///     }
/// }
///
/// impl From<PolygonMode> for GLItemSingle { ... }
/// impl From<PolygonMode> for u32 { ... }
/// // Same impls for OtherGlThing
///
/// ```
pub fn gl_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as GLEnums);
    //quote! {}
    todo!()
}

struct GLEnums {
    enums: Vec<GLEnum>,
}
struct GLEnum {
    pub name: Ident,
    pub variant_idents: Vec<Ident>,
}
impl Parse for GLEnums {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut enums = Vec::with_capacity(3);
        while !input.is_empty() {
            let mut variant_idents = Vec::with_capacity(20);
            let name: Ident = input.parse()?;
            let content;
            let _brace = braced!(content in input);
            let variants = content.parse_terminated(Variant::parse, Token![,])?;
            for var in variants.into_iter() {
                variant_idents.push(var.ident);
            }
            enums.push(GLEnum {
                name,
                variant_idents,
            })
        }
        Ok(Self { enums })
    }
}
