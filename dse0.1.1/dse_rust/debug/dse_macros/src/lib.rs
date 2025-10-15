///////////////////////////////////////////////////
///                                             ///
///  REMEMBER RE SRART IDE AFTER CODING FINISH  ///
///                                             ///
///  编写完成后重启IDE  rust-analyser不是很聪明   ///
///                                             ///
///////////////////////////////////////////////////

/// 宏编程规范 （写给自己看的，不具有强制性）
/// 毕竟对于开发者来说，当然是怎么舒服怎么来
/// 当然个人建议还是给自己的宏代码定一个规范
/// 一方面 方便debug 以及 更改名称
/// 另一方面 免得过段时间 忘了自己写了什么东西了
/// 好吧 实话实说不写其实也没关系
/// 毕竟这玩意都是专门的结构专门的用途，
/// 大部分情况下基本不会去改动（无奈）
///
/// xxx_token: 定义&调用 表述 (后面未标明的 默认为接token)
/// xxx_ident: 字段&命名 表述
/// self_ast: 结构体的 AST树
/// self_ident: 结构体 自身命名
/// title_xxx: 结构体实行 开头定义阶段的表述
/// title_fn_xxx: 函数 开头定义阶段的表述
/// fn_xxx: 函数表述
/// major_xxx : 具体实现表述
/// major_fn_xxx: 具体函数实现表述
/// iter_xxx: 由AST树产生的 迭代表述
/// code_out: 最终代码表述输出
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{self, format_ident, ToTokens};
use syn::{parse_macro_input, DeriveInput};
mod tool;

#[cfg(feature = "config_META_VALUE_TYPE_NUM_32")]
static META_VALUE_TYPE_NUM: i32 = 32;

///
///
///
#[proc_macro_derive(ExecuteMImplement)]
pub fn derive_execute_m_implement(input: TokenStream) -> TokenStream {
    let self_ast = parse_macro_input!(input as DeriveInput);

    let self_ident = self_ast.ident;

    let title_self = quote::quote! {impl #self_ident};

    let title_fn_id_sort = quote::quote! {pub fn id_sort(&mut self)};
    let mut major_fn_id_sort = quote::quote! {};

    match self_ast.data {
        syn::Data::Struct(val) => {
            let mut count :u64 = 0;
            for f in val.fields.iter() {
                let iter_ident = f.ident.as_ref().unwrap();
                major_fn_id_sort = quote::quote! {
                    #major_fn_id_sort
                    self.#iter_ident.set_id(#count);
                };
                count = count + 1;
            }
        }
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    }
    let fn_id_sort_ident = quote::quote! {
        #title_fn_id_sort
        {
            #major_fn_id_sort
        }
    };

    let code_out = quote::quote! {
        #title_self
        {
            #fn_id_sort_ident
        }
    };

    return proc_macro::TokenStream::from(code_out);
}

/// 静态序列化宏
/// 将 结构体数据序列化为Toml数据
///
///
///
#[proc_macro_derive(MetaSerialize2Toml)]
pub fn derive_meta_serialize2toml(input: TokenStream) -> TokenStream {
    let self_ast = parse_macro_input!(input as DeriveInput);

    let self_ident = self_ast.ident;

    let title_self = quote::quote! {impl crate::meta::env::MetaSerialize2Toml for #self_ident};

    let title_fn_to_toml = quote::quote! {
        fn to_toml(&self) -> String
    };

    // for fi in self_ast.

    let major_fn_to_toml = quote::quote! {};

    let fn_to_toml = quote::quote! {
        #title_fn_to_toml{
            #major_fn_to_toml
        }
    };

    let code_out = quote::quote! {
        #title_self{
            #fn_to_toml
        }
    };

    return proc_macro::TokenStream::from(code_out);
}

#[proc_macro_derive(MetaDeSerializeFromToml)]
pub fn derive_meta_deserialize_from_toml(input: TokenStream) -> TokenStream {
    let self_ast = parse_macro_input!(input as DeriveInput);

    let self_ident = self_ast.ident;

    let title_self = quote::quote! {impl #self_ident};

    let code_out = quote::quote! {};

    return proc_macro::TokenStream::from(code_out);
}

/// BufferMImplement proc macro
/// 一次性缓存管理器 过程宏
/// abstract：
/// Add GC function for BufferM Auto.
/// 用途：
/// 自动化 为一次性缓存 添加 垃圾回收函数
/// implement：
///
/// 作用：
///

#[proc_macro_derive(BufferMImplement)]
pub fn derive_buffer_m_implement(input: TokenStream) -> TokenStream {
    let self_ast = parse_macro_input!(input as DeriveInput);

    let self_ident = self_ast.ident;

    let title_self = quote::quote! {impl #self_ident};

    let title_fn_release = quote::quote! {pub fn release(&mut self)};
    let mut major_fn_release = quote::quote! {};
    match self_ast.data {
        syn::Data::Struct(val) => {
            for f in val.fields.iter() {
                let iter_ident = f.ident.as_ref().unwrap();
                major_fn_release = quote::quote! {
                    #major_fn_release
                    self.#iter_ident.release_buffer();
                }
            }
        }
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    }
    let fn_release_ident = quote::quote! {
        #title_fn_release
        {
            #major_fn_release
        }
    };

    let code_out = quote::quote! {
        #title_self
        {
            #fn_release_ident
        }
    };

    return proc_macro::TokenStream::from(code_out);
}

/*

//
#[cfg(feature = "load_meta_serialize")]
#[proc_macro]
pub fn proc_macro_build_meta_inner_iter_type_num(input: TokenStream)-> TokenStream {
    let input:usize=input.to_string().parse().unwrap();
    let mut code_out = quote::quote! {};
    for i in 0..input{
        let ValueType_ident=format_ident!("ValueType{}",i);
        code_out=quote::quote! {
            #code_out
            type #ValueType_ident: Default + MetaBase + Sized;
        };
    }
    return proc_macro::TokenStream::from(code_out);
}

//
#[cfg(feature = "load_meta_serialize")]
#[proc_macro]
pub fn proc_macro_build_vec_inner_meta(input: TokenStream)-> TokenStream{

    let mut code_out = quote::quote! {};
    return proc_macro::TokenStream::from(code_out);
}


/// 过时的
///
///
///
//
#[cfg(feature = "load_meta_serialize")]
#[proc_macro_derive(MetaInnerIter)]
pub fn derive_meta_inner_iter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let self_ident = input.ident;

    let title_ident = quote::quote! {
        impl crate::meta_serialize::MetaInnerIter for #self_ident
    };

    let mut major_ident1 = quote::quote! {};

    let mut ty_count = 0;
    match input.data {
        syn::Data::Struct(s_val) => {
            for (index, val) in s_val.fields.iter().enumerate() {
                ty_count = ty_count + 1;
                let iter_ident = format_ident!("ValueType{}", index);
                let ty_part = val.ty.to_token_stream();
                major_ident1 = quote::quote! {
                    #major_ident1
                    type #iter_ident = #ty_part;
                };
            }
        }
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    };

    let mut major_ident2 = quote::quote! {};

    for i in ty_count..META_VALUE_TYPE_NUM {
        let iter_ident = format_ident!("ValueType{}", i as usize);
        major_ident2 = quote::quote! {
                #major_ident2
                type #iter_ident =Option<bool>;
        };
    }

    let code_out = quote::quote! {
        #title_ident
        {
            #major_ident1
            #major_ident2
        }
    };

    return proc_macro::TokenStream::from(code_out);
}



/// 过时的
///
///
///
//
#[cfg(feature = "load_meta_serialize")]
#[proc_macro_derive(BuildCustomStructMeta)]
pub fn derive_build_custom_struct_meta(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let input_ident = input.ident;

    let innermeta_ident=tool::replace_key_char_into_ident(input_ident.to_string());
    let innermeta_ident=format_ident!("{}_{}", innermeta_ident, tool::META_TABLE_POSTFIX);

    // _innermeta struct build
    let code_ident1;
    match input.data {
        syn::Data::Struct(ref val) => {
            let title_ident1 = quote::quote! {struct #innermeta_ident};
            let mut major_ident1 = quote::quote! {};
            for f in val.fields.iter() {
                let iter_ident1 = f.ident.as_ref().unwrap();
                let ty1=f.ty.to_token_stream();
                let ty1_table = tool::replace_key_char_into_ident(ty1.to_string());
                let ty1_table = format_ident!("{}_{}", ty1_table, tool::META_TABLE_POSTFIX);

                match tool::assert_token_type(ty1.to_string()) {
                    tool::TokenType2InnerMeta::CustomStruct => {
                        major_ident1 = quote::quote! {
                            #major_ident1
                            #iter_ident1 :crate::meta_serialize::MetaD<#ty1_table>,
                        };
                    },
                    tool::TokenType2InnerMeta::Preimplement => {
                        major_ident1 = quote::quote! {
                            #major_ident1
                            #iter_ident1 :crate::meta_serialize::MetaD<#ty1>,
                        };
                    },
                    tool::TokenType2InnerMeta::Array => {

                        major_ident1 = quote::quote! {
                            #major_ident1
                            #iter_ident1 :crate::meta_serialize::MetaD<#ty1>,
                        };
                    },
                    tool::TokenType2InnerMeta::Vector => {
                        major_ident1 = quote::quote! {
                            #major_ident1
                            #iter_ident1 :crate::meta_serialize::MetaD<#ty1>,
                        };
                    }
                    tool::TokenType2InnerMeta::None => todo!(),
                }
            }

            code_ident1 = quote::quote! {
                #title_ident1
                {
                    #major_ident1
                }
            };
        }
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    }

    // _innermeta MetaBase default impl
    let code_ident2;
    match input.data {
        syn::Data::Struct(ref val) => {
            let title_ident1 = quote::quote! {impl Default for #innermeta_ident};
            let sub_title_ident1 = quote::quote! { fn default() -> #innermeta_ident};
            let mut major_ident1 = quote::quote! {};
            for f in val.fields.iter() {
                let iter_ident1 = f.ident.as_ref().unwrap();

                major_ident1 = quote::quote! {
                    #major_ident1
                    #iter_ident1 :Default::default(),
                };
            }
            code_ident2 = quote::quote! {
                #title_ident1
                {
                    #sub_title_ident1{
                        return #innermeta_ident{
                            #major_ident1
                        }
                    }
                }
            };
        }
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    }

    // _innermeta MetaBase impl
    let title_ident3 = quote::quote! {impl crate::meta_serialize::MetaBase for #innermeta_ident};
    let major_ident3 = quote::quote! {
        type ValueType = Option<bool>;
        fn is_custom_struct() -> bool {
            todo!()
        }
    };

    let code_ident3 = quote::quote! {
        #title_ident3
        {
            #major_ident3
        }
    };

    // _innermeta Deserialize impl
    let title_ident4 = quote::quote! {impl crate::meta_serialize::DeSerialize for crate::meta_serialize::MetaD<#innermeta_ident>};
    let mut major_ident4  = quote::quote! {};
    major_ident4 = quote::quote! {
        #major_ident4
        type TargetType = #input_ident;
    };
    let mut fn_deserialize_ident=quote::quote!{};
    let fn_deserialize_title=quote::quote!{fn deserialize(self) -> Result<Self::TargetType, String>};
    let mut iter_ident=quote::quote!{};
    match input.data {
        syn::Data::Struct(val) => {
            for f in val.fields.iter(){
                let name=f.ident.as_ref().unwrap();
                iter_ident=quote::quote!{
                    #iter_ident
                    #name : self.value.#name.deserialize().unwrap(),
                };
            }
        },
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    }

    fn_deserialize_ident=quote::quote!{
        #fn_deserialize_title
        {
            return Ok(
                #input_ident
                {
                    #iter_ident
                }
            );
        }
    };

    major_ident4 = quote::quote! {
        #major_ident4
        #fn_deserialize_ident
    };

    let code_ident4 = quote::quote! {
        #title_ident4
        {
            #major_ident4
        }
    };


    let code_out = quote::quote! {
        #code_ident1
        #code_ident2
        #code_ident3
        #code_ident4
    };
    return proc_macro::TokenStream::from(code_out);
}


/// 过时的
///
///
///
//
#[cfg(feature = "load_meta_serialize")]
#[proc_macro_derive(MetaBasePreImplement)]
pub fn derive_meta_base_pre_implement(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let self_ident = input.ident;

    //out
    let code_out = quote::quote! {};

    return proc_macro::TokenStream::from(code_out);
}


/// 过时的
///
///
///
//
#[cfg(feature = "load_meta_serialize")]
#[proc_macro_derive(MetaBaseCustomStruct)]
pub fn derive_meta_base_custom_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let self_ident = input.ident;

    // MetaBase impl
    let code_ident1;
    let title_ident2 = quote::quote! {impl crate::meta_serialize::MetaBase for #self_ident};

    let sub_ident1=tool::replace_key_char_into_ident(self_ident.to_string());
    let sub_ident1 = format_ident!("{}_{}", sub_ident1, tool::META_TABLE_POSTFIX);

    // serialize table fn AST
    let mut fn_serialize_table_major = quote::quote! {};
    match input.data {
        syn::Data::Struct(val) => {
            for f in val.fields.iter() {
                let iter_ident1 = f.ident.as_ref().unwrap();
                let iter_ty1 = f.ty.to_token_stream();
                //let iter_ty1_table= format_ident!("{}_{}",iter_ty1,tool::META_TABLE_POSTFIX);

                // WARNING: code below use FUNCTION NESTING in macro witch
                // will cause compile_time stack over even editor crash on theory
                // recommand your target deep level is less than 16 to reach end!
                match tool::assert_token_type(iter_ty1.to_string()) {
                    tool::TokenType2InnerMeta::CustomStruct => {
                        fn_serialize_table_major = quote::quote! {
                            #fn_serialize_table_major
                            #iter_ident1: <#iter_ty1>::serialize(self.#iter_ident1, stringify!(#iter_ident1).to_string(), 0)
                                .expect_custom_struct()
                                .unwrap(),
                        };
                    },
                    tool::TokenType2InnerMeta::Preimplement => {
                        fn_serialize_table_major = quote::quote! {
                            #fn_serialize_table_major
                            #iter_ident1: <#iter_ty1>::serialize(self.#iter_ident1, stringify!(#iter_ident1).to_string(), 0)
                                .expect_pre_implement()
                                .unwrap(),
                        };
                    },
                    tool::TokenType2InnerMeta::Array => {
                        fn_serialize_table_major = quote::quote! {
                            #fn_serialize_table_major
                            #iter_ident1: <#iter_ty1>::serialize(self.#iter_ident1, stringify!(#iter_ident1).to_string(), 0)
                                .expect_pre_implement()
                                .unwrap(),
                        };
                    },
                    tool::TokenType2InnerMeta::Vector => {
                        fn_serialize_table_major = quote::quote! {
                            #fn_serialize_table_major
                            #iter_ident1: <#iter_ty1>::serialize(self.#iter_ident1, stringify!(#iter_ident1).to_string(), 0)
                                .expect_pre_implement()
                                .unwrap(),
                        };
                    },
                    tool::TokenType2InnerMeta::None => todo!(),
                }

            }
            fn_serialize_table_major = quote::quote! {
                return Ok(#sub_ident1
                {
                    #fn_serialize_table_major
                });
            };
        }
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    };

    let major_ident2 = quote::quote! {
        type ValueType = #sub_ident1;
        fn is_custom_struct() -> bool {
            return true;
        }

        fn serialize_table(mut self) -> Result<#sub_ident1, String> {
            #fn_serialize_table_major
        }
    };

    code_ident1 = quote::quote! {
        #title_ident2
        {
            #major_ident2
        }
    };

    //out
    let code_out = quote::quote! {
        #code_ident1
    };
    return proc_macro::TokenStream::from(code_out);
}




/// 过时的
///
///
///
#[cfg(feature = "load_meta_serialize")]
#[proc_macro_derive(MetaTableDesrialize)]
pub fn derive_meta_table_desrializ(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let self_ident = input.ident;



    let title_token=quote::quote!{impl crate::meta_serialize::DeSerialize for #self_ident};

    let code_out=quote::quote!{
        #title_token
        {

        }
    };
    return TokenStream::from(code_out);
}

//
#[cfg(feature = "load_meta_serialize")]
#[proc_macro]
pub fn proc_macro_build_array_impl_metabase(input: TokenStream)-> TokenStream {
    let input:usize=input.to_string().parse().unwrap();
    let mut code_out = quote::quote! {};
    for i in 0..input{
        let count=i+1;
        code_out=quote::quote! {
            #code_out
            impl<T> crate::meta_serialize::MetaBase for [T;#count]
            where T:Default+crate::meta_serialize::MetaBase{
                type ValueType = T;
                fn is_custom_struct() -> bool {
                    return false;
                }
            }
        };
        code_out=quote::quote!{
            #code_out
            impl<T> crate::meta_serialize::MetaBasePreImplement for [T;#count]
            where T:Default+crate::meta_serialize::MetaBase {}
        }
    }
    return proc_macro::TokenStream::from(code_out);
}



*/

#[cfg(feature = "test_dse_proc_macro")]
#[proc_macro]
pub fn dse_proc_macro_example(input: TokenStream) -> TokenStream {
    let mut code: String = String::default();
    let input_str = format!("{}", input);
    code = code + (r#"println!("{:?}","#) + input_str.as_str() + ");";
    code = code + (r#"println!("{:?}","#) + input.to_string().as_ref() + ");";
    return code.parse().unwrap();
}

#[cfg(feature = "test_dse_proc_macro")]
#[proc_macro]
pub fn dse_proc_macro_ext_test(input: TokenStream) -> TokenStream {
    let input_str = format!("{}", input);
    let code = quote::quote!(
        println!("hahah");
    );
    let mut code_out = quote::quote!();
    for i in 0..3 {
        let part = quote::format_ident!("part{}", i as usize);
        code_out = quote::quote!(
        println!("{}",#i);
        #code
        #code_out
        println!("{:?}",#input_str);
        let #part = 3;
        dbg!(#part);
        );
    }

    return proc_macro::TokenStream::from(code_out);
}
