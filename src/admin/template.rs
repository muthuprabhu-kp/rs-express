use tera::Tera;
use tera::Context;
use include_dir::{include_dir, Dir};
use std::path::Path;
use std::fs;

pub fn load_page(param: (bool, Vec<String>)) -> (u16,String) {
    let parent_path = format!("./{}/*.html", param.1[0]);

    println!("{}",parent_path);

    let file_path = format!("./{}.html", param.1.join("/"));
    let template_string = fs::read_to_string(file_path.clone());
    //const PROJECT_DIR: Dir = include_dir!(parent_path);
    let mut tera = Tera::default();
    let respose = tera.add_raw_template(file_path.as_str(), template_string.unwrap().as_str());
    (200,tera.render(file_path.as_str(), &Context::new()).unwrap())
}

