extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};
use proc_macro::Ident;
use litrs;

use std::convert::TryFrom;
#[proc_macro]
pub fn sheet_assets(item: TokenStream) -> TokenStream {

    let mut iter = item.clone().into_iter();

    let name = match iter.next().unwrap() {
        TokenTree::Ident(ident) => {
            ident.to_string()
        },
        other => {
            panic!("Expected Ident as first argument to sheet_assests!, got :\n{:#?}", other);
        }
    };

    let second_token = iter.next().unwrap();
    let path = match litrs::StringLit::try_from(second_token) {
        Ok(string_lit) => string_lit.value().to_string(),
        Err(e) => return e.to_compile_error(),
    };


    let mut res = format!("#[derive(Debug, Clone, Copy)] struct {name} {{\n");

    let dir = match std::fs::read_dir(&path) {
        Ok(d) => d,
        Err(err) => {
            println!("Path was '{:?}'", &path);
            panic!("{}",err);
        }
    };


    let mut json_files = std::collections::HashSet::new();
    let mut png_files = std::collections::HashSet::new();

    let mut res = format!("#[derive(Debug)] struct {name} {{\n");

    for dir_entry in dir {
        match dir_entry {
            Ok(e) => {
                let file_type = e.metadata().unwrap().file_type();
                if file_type.is_file() {

                    let file_name = e.file_name().into_string().unwrap();

                    let file_name_no_ending = file_name.split(".").next().unwrap().to_string();

                    if file_name.ends_with(".json") {
                        json_files.insert(file_name_no_ending.clone());
                    }

                    if file_name.ends_with(".png") {
                        png_files.insert(file_name_no_ending.clone());
                    }
                }
            },
            _ => {}
        }
    }

    let mut names = std::collections::HashSet::new();

    for json in &json_files {
        if png_files.contains(json) {
            res += &format!("pub {}: gl_lib::animations::sheet_animation::SheetAnimation,\n", json.to_lowercase());
            names.insert(json.clone());

        }
    }



    let end = "}\n";

    res += end;

    res += &format!("impl {name} {{\n");
    add_load_all(&mut res, &name, &names);

    add_all_names(&mut res, &name, &names);


    // impl close
    res += "}\n";

    //println!("{}", res);
    res.parse().unwrap()

}

fn add_all_names(res: &mut String, name: &str, names: &std::collections::HashSet::<String>) {

    *res += "pub fn all_names(&self) -> Vec::<(&str, &gl_lib::animations::sheet_animation::SheetAnimation)>{\n";
    *res += "vec![\n";


    for field_name in names {
        *res += &format!("(\"{}\", &self.{}),\n", field_name.to_lowercase(), field_name.to_lowercase())

    }

    *res += "]\n";


    // pub fn {
    *res += "}\n";
}

fn add_load_all(res: &mut String, name: &str, names: &std::collections::HashSet::<String>) {

    *res += &format!("pub fn load_all(gl: &gl_lib::gl::Gl, path: &str) -> {name} {{\n");
    *res += &format!("let mut id = 1;\n");
    *res += &format!("{name} {{\n ");

    for field_name in names {
        *res += &format!("{}: gl_lib::animations::sheet_animation::load_by_name(gl, &std::path::Path::new(path), &\"{field_name}\", &mut id),\n", field_name.to_lowercase())

    }

    // {name} {
    *res += "}\n";

    // pub fn {
    *res += "}\n";



}
