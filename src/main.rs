use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::Path;
use std::process::Command;

use savefile::prelude::*;
use colored::Colorize;

const FILENAME:&str = "mapdata.bin";

struct ModifiedPkg {
    name: String,
    version: String,
    tag: String
}

fn save_hashmap(hashmap:&HashMap<String,String>) {
    save_file(FILENAME, 0, hashmap).unwrap();
}

fn load_hashmap() -> Result<HashMap<String, String>,savefile::SavefileError> {
    load_file(FILENAME, 0)
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

fn generate_hashmap_from_file() -> HashMap<String, String> {
    match load_hashmap() {
        Ok(hm) => hm,
        Err(_) => {
            let path = Path::new(FILENAME);
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

fn query() {
    let hm:HashMap<String, String> = generate_hashmap_from_file();
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
        println!("{} {} | {}", i.name.bold(), i.version.bold(), i.tag.italic().green());
    }

    save_hashmap(&hm);

}

fn tag(args:Vec<String>) {
    let pkg = &args[2];
    let tag = &args[3];

    // Real hashmap
    let mut hm:HashMap<String, String> = generate_hashmap_from_file();

    // Fake hashmap to make sure package is really in pacman
    let mut all_packages = HashMap::new();
    let vec:Vec<String> = get_pacman_pkgs();
    for pkg in vec.iter() {
        all_packages.insert(pkg.to_string(), "".to_string());
    }
    if all_packages.contains_key(pkg) {
        hm.insert(pkg.to_string(), tag.to_string());
        save_hashmap(&hm);
        println!("Saved {} with tag: {}", pkg, tag);
    }
    else {
        println!("Sorry! {} isn't in your list of packages!", pkg);
        return
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1{
        println!("Usage: pactag -Q | -L");
        return;
    }
    let flag:&str = &args[1];
    match flag {
        "-Q" => {
            if args.len() > 2 {
                println!("Usage: pactag -Q");
                return;
            }
            query();
        },
        "-L" => {
            if args.len() <= 3 {
                println!("Usage: pactag -L [package] [tags]");
                return;
            }
            tag(args);
        },
        _ => println!("Usage: pactag -Q | -L"),
    }
}
