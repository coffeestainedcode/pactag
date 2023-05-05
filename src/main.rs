use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::Path;
use std::process::Command;

use savefile::prelude::*;
use colored::Colorize;

// const FILENAME:&str = "mapdata.bin";

struct ModifiedPkg {
    name: String,
    version: String,
    tag: String
}

fn save_hashmap(hashmap:&HashMap<String,String>, path:&String) {
    save_file(path, 0, hashmap).unwrap();
}

fn load_hashmap(path:&String) -> Result<HashMap<String, String>,savefile::SavefileError> {
    load_file(path, 0)
}

fn get_pacman_pkgs() -> Vec<String> {
    let pacman = String::from_utf8(
        Command::new("pacman").arg("-Qe").output().expect("Pacman fail").stdout
    );
    let packages = match pacman {
        Ok(pkgs) => pkgs,
        Err(_) => panic!("Bad bad bad")
    };
    let mut out: Vec<String> = Vec::new();
    for pkgs in packages.lines() {
        let pkg_name:String = match pkgs.split_whitespace().next() {
            Some(val) => val.to_string(),
            None => panic!("AHHH")
        };
        out.push(pkg_name);
    }
    out
}

fn generate_hashmap_from_file(path:&String) -> HashMap<String, String> {
    match load_hashmap(&path) {
        Ok(hm) => hm,
        Err(_) => {
            let path = Path::new(&path);
            let display = path.display();
            match File::create(&path) {
                Ok(..) => {
                    let mut hm = HashMap::new();
                    let vec:Vec<String> = get_pacman_pkgs();
                    for pkg in vec.iter() {
                        hm.insert(pkg.to_string(), "".to_string());
                    }
                    hm
                }
                Err(why) => panic!("couldn't create {}: {}", display, why),
            }
        }
    }
}

fn query(path:&String) {
    let hm:HashMap<String, String> = generate_hashmap_from_file(&path);
    let pacman = String::from_utf8(
        Command::new("pacman").arg("-Qe").output().expect("Pacman fail").stdout
    );
    let packages = match pacman {
        Ok(pkgs) => pkgs,
        Err(_) => panic!("Bad bad bad")
    };
    let mut out: Vec<ModifiedPkg> = Vec::new();
    for pkgs in packages.lines() {
        let mut pkg_iterator = pkgs.split_whitespace();
        let pkg_name = match pkg_iterator.next() {
            Some(val) => val.to_string(),
            None => panic!("AHHH")
        };
        let pkg_version = match pkg_iterator.next() {
            Some(val) => val.to_string(),
            None => panic!("AHHH")
        };
        if hm.contains_key(&pkg_name) {
            let hm_value:String = match hm.get(&pkg_name) {
                Some(val) => val.to_string(),
                None => panic!("What?")
            };
            out.push(ModifiedPkg {
                name: pkg_name,
                version: pkg_version,
                tag: hm_value
            });
        }
        else {
            out.push(ModifiedPkg{
                name: pkg_name,
                version: pkg_version,
                tag: "".to_string()
            });
        }
    }
    for i in out.iter() {
        if i.tag == "" {
            println!("{} {}", i.name.bold(), i.version.bold());
        }
        else {
            println!("{} {} | {}", i.name.bold(), i.version.bold(), i.tag.italic().green());
        }
    }

    save_hashmap(&hm, &path);

}

fn tag(path:&String, args:Vec<String>) {
    let pkg = &args[2];
    let tag = &args[3];

    // Real hashmap
    let mut hm:HashMap<String, String> = generate_hashmap_from_file(&path);

    // Fake hashmap to make sure package is really in pacman
    let mut all_packages = HashMap::new();
    let vec:Vec<String> = get_pacman_pkgs();
    for pkg in vec.iter() {
        all_packages.insert(pkg.to_string(), "".to_string());
    }
    if all_packages.contains_key(pkg) {
        hm.insert(pkg.to_string(), tag.to_string());
        save_hashmap(&hm, &path);
        println!("Saved {} with tag: {}", pkg.bold().green(), tag.bold().green());
    }
    else {
        println!("Sorry! {} isn't in your list of packages!", pkg);
        return
    }
}

fn remove(path:&String, args:Vec<String>) {
    let pkg = &args[2];
    let mut hm:HashMap<String, String> = generate_hashmap_from_file(&path);
    if hm.contains_key(pkg) {
        hm.insert(pkg.to_string(), "".to_string());
    }
    save_hashmap(&hm, &path);
    println!("Removed tag from {}", pkg.bold().red());
    return;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1{
        println!("Usage: pactag -Q | -L");
        return;
    }
    let mut path = match env::var("XDG_CACHE_HOME") {
        Ok(val) => val.to_string(),
        Err(_) => "~/.cache".to_string()
    };
    path.push_str("/pactag.db");

    let flag:&str = &args[1];
    match flag {
        "-Q" => {
            if args.len() > 2 {
                println!("Usage: pactag -Q");
                return;
            }
            query(&path);
        },
        "-L" => {
            if args.len() <= 3 || args.len() > 4 {
                println!("Usage: pactag -L [package] [tag]");
                return;
            }
            tag(&path, args);
        },
        "-R" => {
            if args.len() <= 2 || args.len() > 3 {
                println!("Usage: pactag -R [package]");
                return;
            }
            remove(&path, args);
        }
        _ => println!("Usage: pactag -Q | -L"),
    }
}
