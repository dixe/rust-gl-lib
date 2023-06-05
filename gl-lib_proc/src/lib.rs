extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};
use proc_macro::Ident;
use litrs;

use std::convert::TryFrom;
#[proc_macro]
pub fn sheet_assets(item: TokenStream) -> TokenStream {

    let mut iter = item.clone().into_iter();

    println!("{:#?}", item);

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
            res += &format!("pub {json}: gl_lib::animations::sheet_animation::SheetAnimation,\n");
            names.insert(json.clone());
            println!("{:?}", json);
        }
    }



    let end = "}\n";

    res += end;

    res += &format!("impl {name} {{\n");
    add_load_all(&mut res, &name, &names);

    add_load_by_name(&mut res);


    println!("{}", res);

    res.parse().unwrap()

}

fn add_load_by_name(res: &mut String) {
    *res += include_str!("E:/repos/rust-gl-lib/gl-lib_proc/src/load_by_name.rs");
}

fn add_load_all(res: &mut String, name: &str, names: &std::collections::HashSet::<String>) {



    *res += &format!("pub fn load_all(ui: &mut gl_lib::imode_gui::ui::Ui, path: &str) -> {name} {{\n");

    *res += &format!("{name} {{\n ");

    for field_name in names {
        *res += &format!("{field_name}: {name}::load_by_name(ui, &std::path::Path::new(path).join(\"{field_name}.json\")),\n")

    }

    // {name} {
    *res += "}\n";

    // pub fn {
    *res += "}\n";



}
