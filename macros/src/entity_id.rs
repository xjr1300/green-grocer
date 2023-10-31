use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{self, DeriveInput};

pub(crate) fn derive_entity_id_impl(input: DeriveInput) -> syn::Result<TokenStream2> {
    let ident = input.ident;

    Ok(quote! {
        impl #ident {
            /// エンティティIDをUUIDで返却する。
            ///
            /// # 戻り値
            ///
            /// エンティティID
            pub fn value(&self) -> uuid::Uuid {
                self.value
            }
        }

        impl From<uuid::Uuid> for #ident {
            /// UUIDからエンティティIDを構築する。
            ///
            /// # 引数
            ///
            /// * `value` - UUID
            ///
            /// # 戻り値
            ///
            /// エンティティID
            fn from(value: uuid::Uuid) -> Self {
                Self {
                    value,
                }
            }
        }

        impl TryFrom<&str> for #ident {
            type Error = uuid::Error;

            /// 文字列からエンティティIDを構築する。
            ///
            /// # 引数
            ///
            /// * `value` - エンティティIDを構築する文字列
            ///
            /// # 戻り値
            ///
            /// エンティティID
            ///
            /// # エラー
            ///
            /// uuid::Error
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                let value = uuid::Uuid::parse_str(value)?;

                Ok(Self::from(value))
            }
        }
    })
}
