#![allow(non_snake_case)]
extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};
use litrs;
use walkdir::WalkDir;
use std::convert::TryFrom;

#[derive(Debug)]
enum JsonNames {
    NotValid,
    Names(Vec::<String>),
    UseFileName
}

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

    let mut res = "".to_string();
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir()) {
            res += "\n\n";
            res += &create_struct_from_folder(entry.path(), &name);
        }


    //println!("{}", res);
    res.parse().unwrap()
}


fn upper_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}


fn create_struct_from_folder(path: &std::path::Path, base_name: &str) -> String {

    // crate name from base_name and folder name, fx PlayerAssets
    let dir_name = path.file_name().unwrap().to_str().unwrap();
    let name = format!("{}{}", upper_first_letter(dir_name), base_name);

    let dir = match std::fs::read_dir(&path) {
        Ok(d) => d,
        Err(err) => {
            println!("Path was '{:?}'", &path);
            panic!("{}",err);
        }
    };


    let mut json_files = std::collections::HashSet::<String>::new();

    let mut asset_names = std::collections::HashSet::<String>::new();


    let mut res = format!("#[derive(Debug)]\n pub struct {name} {{\n");

    for dir_entry in dir {
        match dir_entry {
            Ok(e) => {
                let file_type = e.metadata().unwrap().file_type();
                if file_type.is_file() {

                    let file_name = e.file_name().into_string().unwrap();
                    let file_name_no_ending = file_name.split(".").next().unwrap().to_string();

                    if file_name.ends_with(".json") {
                        let json_names = load_json_animation_names(&e.path());

                        match json_names {
                            JsonNames::Names(v) => {
                                json_files.insert(file_name_no_ending.clone());
                                for name in &v {
                                    asset_names.insert(name.to_string());
                                }
                            },
                            JsonNames::UseFileName => {
                                asset_names.insert(file_name_no_ending.clone());
                                json_files.insert(file_name_no_ending.clone());
                            }
                            JsonNames::NotValid => { continue;}
                        }
                    }
                }
            },
            _ => {}
        }
    }


    for asset_name in &asset_names {
        res += &format!("{asset_name}: gl_lib::animations::sheet_animation::SheetAnimation,\n");
    }


    let end = "}\n";

    res += end;

    res += &format!("impl {name} {{\n");

    add_load_all(&mut res, &name, &json_files, &asset_names, &dir_name);

    add_all_names(&mut res, &asset_names);


    // impl close
    res += "}\n\n\n";

    res
}



/// Load json file, and extrat animation names
fn load_json_animation_names(path: &std::path::Path) -> JsonNames {

    let anim_json = std::fs::read_to_string(path).unwrap();


    let sheet_anim : SheetArrayAnimation = match serde_json::from_str(&anim_json) {
        Ok(sheet) => {
            sheet
        },
        Err(_) => {
            return JsonNames::NotValid;
        }
    };
    if sheet_anim.meta.frameTags.len () == 0 {
            return JsonNames::UseFileName;
    }

    let mut names = vec![];
    for tag in &sheet_anim.meta.frameTags {
        names.push(tag.name.clone());
    }


    JsonNames::Names(names)
}


fn add_all_names(res: &mut String, names: &std::collections::HashSet::<String>) {

    *res += "pub fn all_names(&self) -> Vec::<(&str, &gl_lib::animations::sheet_animation::SheetAnimation)>{\n";
    *res += "vec![\n";


    for field_name in names {
        *res += &format!("(\"{}\", &self.{}),\n", field_name.to_lowercase(), field_name.to_lowercase())

    }

    *res += "]\n";


    // pub fn {
    *res += "}\n";
}

use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
struct SheetArrayAnimation {
    pub meta: Meta,
}


#[derive(Default, Debug, Serialize, Deserialize)]
struct Meta {
    pub frameTags: Vec::<FrameTag>
}


#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct FrameTag {
    pub name: String,
}



fn add_load_all(res: &mut String, struct_name: &str, file_names: &std::collections::HashSet::<String>, asset_names: &std::collections::HashSet::<String>, dir_name: &str) {

    *res += &format!("pub fn load_all(gl: &gl_lib::gl::Gl, path: &str) -> {struct_name} {{\n");
    *res += &format!("let mut id = 1;\n");

    *res += "let mut assets = std::collections::HashMap::<String, gl_lib::animations::sheet_animation::SheetAnimation>::new();\n\n";

    *res += "let mut pb = std::path::PathBuf::new();\n";

    *res += "pb.push(path);\n";

    *res += &format!("pb.push(\"{dir_name}\");\n");

    for file_name in file_names {

        *res += &format!("pb.push(\"{file_name}.json\");\n");

        *res += &format!("\nfor asset in gl_lib::animations::sheet_animation::load_by_name(gl, &pb, \"{file_name}\", &mut id).unwrap() {{
            assets.insert(asset.name.clone(), asset.clone());\n}}\n");
        *res += "pb.pop();\n";
    }


    *res += &format!("{struct_name} {{\n ");

    for field_name in asset_names {
        *res += &format!("{}: assets.remove(\"{field_name}\").unwrap(),\n", field_name.to_lowercase())
    }

    // {name} {
    *res += "}\n";

    // pub fn {
    *res += "}\n";



}
