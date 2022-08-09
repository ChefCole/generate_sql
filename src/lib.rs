extern crate proc_macro;
use proc_macro2::{Ident, Span};
use std::collections::HashMap;

use proc_macro::{TokenStream, TokenTree};

#[derive(Debug,Clone)]
struct StructAttribute{
    name:String,
    value_type:String,
    opt:bool
}

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    let mut index = 0;
    let mut struct_name: String = "".to_string();
    let mut param_map = HashMap::<String,StructAttribute>::new();
    for token_tree in _item.into_iter() {
        
        match token_tree {
            TokenTree::Ident(ident_param) => {
                //获取结构体名称
                if index != 0 {
                    //println!("struct = {:?}",&ident_param.to_string());
                    struct_name = ident_param.to_string();
                }
            }
            TokenTree::Group(group) => {
                //计数
                let mut param_map_index = 0;
                let mut param_type:i8 = 0;
                let mut param_pair:i8 = 0;
                let mut param_name:String = String::new();
                let mut param_value:String = String::new();
                let mut struct_attribute = StructAttribute{
                    name:"".to_string(),
                    value_type:"".to_string(),
                    opt: false
                };
                for group_token_tree in  group.stream().into_iter(){
                    match group_token_tree {
                        TokenTree::Ident(ident_param) => {
                            if param_map_index == 0 && param_type < 2 && param_pair == 0{
                                param_value = ident_param.to_string();
                                struct_attribute.value_type = ident_param.to_string();
                                param_map_index += 1;
                            }
                            if param_map_index == 1 && param_type < 2 && param_pair == 0{
                                struct_attribute.opt = false;
                                param_name = ident_param.to_string();
                                struct_attribute.name = ident_param.to_string();
                                param_map_index += 1;
                            }else {
                                if !(param_type >= 2 && param_pair == 0) {
                                    param_map_index = 1;
                                }
                                //判断类型是否是option
                                if param_value.eq("Option") {
                                    struct_attribute.opt = true;
                                }
                                param_value = ident_param.to_string();  
                                struct_attribute.value_type = ident_param.to_string();
                            }
                            param_type = 0;
                        }
                        TokenTree::Group(_group) => {
                            
                        }
                        TokenTree::Literal(_literal) => {
            
                        }
                        TokenTree::Punct(punct) => {
                            if punct.eq(&':') {
                                param_type += 1;
                            }
                            if punct.eq(&'<') {
                                param_pair += 1;
                            }
                            if punct.eq(&'>') {
                                param_pair -= 1;
                                
                            }
                            
                        }
                    }
                    param_map.insert(param_name.clone(), struct_attribute.clone());
                }
            }
            TokenTree::Literal(_literal) => {

            }
            TokenTree::Punct(_punct) => {

            }
        }
        index += 1;
    }

    let str_name = quote::format_ident!("{}",struct_name);
    
    let mut content= quote::quote!();

    for (_name,attr) in param_map.iter() {
        let ident = Ident::new(attr.name.as_str(), Span::mixed_site());
        let value_type = attr.value_type.clone();
        let name_value = attr.name.clone();
        if attr.opt {
            let con = quote::quote!(
                let value_type:String = #value_type.to_string();
                let name_value:String = #name_value.to_string();
                match &item.#ident {
                    Some(val) => {
                        //names.push_str(#struct_zd_name);
                        match value_type.as_str() {
                            "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "f8" | "f16" | "f32" | "f64" => {
                                if start_index != 0 {
                                    names.push_str(",");
                                    values.push_str(",");
                                }
                                names.push_str(name_value.as_str());
                                values.push_str(val.to_string().as_str());
                                start_index += 1;
                            }
                            "String" => {
                                if start_index != 0 {
                                    names.push_str(",");
                                    values.push_str(",");
                                }
                                names.push_str(name_value.as_str());
                                let mut value = String::new();
                                value.push_str("'");
                                value.push_str(val.to_string().as_str());
                                value.push_str("'");
                                values.push_str(value.as_str());
                                start_index += 1;
                            }
                            "NaiveDateTime" =>{
                                if start_index != 0 {
                                    names.push_str(",");
                                    values.push_str(",");
                                }
                                names.push_str(name_value.as_str());
                                let mut value = String::new();
                                value.push_str("'");
                                value.push_str(val.to_string().as_str());
                                value.push_str("'");
                                values.push_str(value.as_str());
                                start_index += 1;
                            }
                            _ => {
                                println!("这不是i32");
                            }
                        }
                    }
                    None =>{

                    }
                }
            );
            content.extend(con);
        }else{

        }
    }

    let fun_sql = quote::quote!(
        impl #str_name {
            fn insert_sql(item: #str_name,table_name:String) -> String{
                let mut names = String::new();
                let mut values = String::new();
                let mut start_index = 0;
                #content
                format!("insert into {} ({}) values ({})",table_name,names,values)
            }
        }
    );

    TokenStream::from(fun_sql)

}


