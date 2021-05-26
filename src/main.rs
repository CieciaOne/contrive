#![deny(clippy::pedantic)]

use dirs;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::{self, File as FS_File};
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "add" => {
                if args.len() == 4 {
                    let (name, path) = (&args[2], &args[3]);
                    //let path = &args[3];
                    match add_template(name.to_string(), path.to_string()) {
                        Ok(_) => (),
                        Err(e) => println!("Error adding file: {}", e),
                    };
                } else {
                    println!(
                        "Incorrect input please try: contrive add <template_name> <file_path>"
                    );
                }
            }
            "list" => {
                list_templates()?;
            }
            "remove" => {
                if args.len() == 3 {
                    let name = &args[2];
                    let mut data = dirs::data_local_dir().unwrap();

                    data.push("contrive");
                    data.push(name);
                    if data.is_file() {
                        println!("Type: \"{}\" if you want to remove template or \"cancel\" if you want to cancel the operation",name);
                        let mut input = String::new();

                        match std::io::stdin().read_line(&mut input) {
                            Ok(_) => {
                                if input.replace("\n", "") == *name {
                                    match fs::remove_file(&data) {
                                        Ok(_) => (),
                                        Err(e) => println!("Error removing template: {}", e),
                                    };
                                    println!("Template {} removed successfully", name);
                                } else {
                                    println!("Invalid name, please try again");
                                }
                            }
                            Err(error) => println!("error: {}", error),
                        }
                    } else {
                        println!("Incorrect input please try: contrive remove <template_name>");
                        list_templates()?;
                    }
                    //let data_dir = data.as_path();
                } else {
                    println!("Incorrect input please try: contrive remove <template_name>");
                    list_templates()?;
                }
            }
            "help" => {
                println!("{}", help());
            }
            item => {
                let list = load_data()?;
                let dyn_args = &args[2..];
                //let list_vec: Vec<&str> = Vec::new();
                let mut isitthere = 0;
                for entry in list {
                    if let Ok(entry) = entry {
                        match entry.path().file_name() {
                            Some(name) => {
                                if OsString::from(item) == name {
                                    let file = match fs::read_to_string(entry.path()) {
                                        Ok(f) => f,
                                        Err(e) => format!("Error: {}", e),
                                    };
                                    //println!("{:?}", parse(file, dyn_args.to_vec()));
                                    match parse(file, dyn_args.to_vec()) {
                                        Ok(_) => (),
                                        Err(e) => {
                                            println!("Error parsing file {}", e);
                                        }
                                    }
                                    isitthere += 1;
                                }
                                if isitthere == 0 {
                                    println!("Template does not exit or command invalid, try contrive help");
                                }
                            }
                            None => (),
                        }
                    }
                }
            }
        }
    } else {
        println!("{}", help());
    }

    Ok(())
}

#[derive(Deserialize, Serialize, Debug, Clone)]
enum FSObject {
    File(File),
    Directory(Directory),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct File {
    name: String,
    contents: String,
}

impl File {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_contents(&self) -> String {
        self.contents.clone()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Directory {
    name: String,
    contents: Vec<FSObject>,
}

impl Directory {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_contents(&self) -> Vec<FSObject> {
        self.contents.clone()
    }
}

fn load_data() -> std::result::Result<std::fs::ReadDir, std::io::Error> {
    let mut data = dirs::data_local_dir().unwrap();

    data.push("contrive");
    let data_dir = data.as_path();
    //println!("{:?}",data);

    if data_dir.is_dir() {
        let dir = data_dir.read_dir();
        dir
    } else {
        fs::create_dir(data_dir)?;
        let dir = data_dir.read_dir();
        dir
    }
}

fn add_template(name: String, file: String) -> std::io::Result<()> {
    let mut data = dirs::data_local_dir().unwrap();
    data.push(format!("contrive/{}", name));
    if data.is_file() {
        println!("Template with this name already exists, remove this one or try different name");
    } else {
        fs::copy(file, data)?;
        println!("Template added succesfully");
    }
    Ok(())
}

fn help() -> String {
    format!(
        "Construct is a simple utility for automatic creation of directory and file structures.

Usage:
  contrive Argument

Arguments:
  <name> <variable:value> \t- create instance of template in working directory
  add <name> <file path> \t- add template(json) to library
  remove <template_name> \t- remove template from library
  list \t\t\t\t- list template library
  help \t\t\t\t- display this message

Examples:
  contrive add lecture lecture.json
  contrive lecture topic:trigonometry date:3_14_21
  contrive remove lecture
"
    )
}

fn rec(cfg: FSObject, mut path: String) -> std::io::Result<()> {
    match cfg {
        FSObject::File(f) => {
            let mut file = FS_File::create(path.clone() + &f.get_name())?;
            file.write_all(&f.get_contents().as_bytes())?;
            Ok(())
        }
        FSObject::Directory(d) => {
            //let temp = path.clone();
            let local = d.get_name() + "/";
            path += &local;
            fs::create_dir_all(path.clone())?;
            for v in d.get_contents() {
                rec(v, path.clone())?;
            }
            Ok(())
        }
    }
}

fn parse(data: String, dyn_args: Vec<String>) -> Result<(), Box<dyn Error>> {
    //println!("{} {:?}", data,dyn_args);
    //println!("{:?}", &dyn_args);
    let mut data = data;
    dyn_args
        .iter()
        .map(|a| {
            let re = a.split(":").collect::<Vec<&str>>();
            // add check(find and reject on None) for values in file

            data = data.replace(&format!("{{{{{}}}}}", re[0]), re[1]);

            re
        })
        //.collect::<Vec<Vec<&str>>>();
        .for_each(drop);

    //println!("{:?}\n{}", da, data);

    let cfg: FSObject = serde_json::from_str(&data)?;

    let path = "./".to_owned();
    rec(cfg, path)?;
    println!("File structure created succesfully");
    Ok(())
}
fn list_templates() -> std::io::Result<()> {
    println!("Template library:");
    let list = load_data()?;
    for entry in list {
        match entry.unwrap().path().file_name().unwrap().to_str() {
            Some(name) => println!("{}", name),
            None => (),
        }
    }
    Ok(())
}
