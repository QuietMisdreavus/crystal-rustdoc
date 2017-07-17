extern crate syn;

use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use syn::MetaItem::NameValue;
use syn::Lit;

//TODO: de-hardcode manifest dir
const ROOT: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/example");

fn read_all_text<T: AsRef<Path>>(filename: T) -> io::Result<String> {
    let mut ret = String::new();
    let mut file = File::open(filename)?;

    file.read_to_string(&mut ret)?;

    Ok(ret)
}

fn flatten_docs(attrs: &[syn::Attribute]) -> String {
    let mut ret = String::new();

    for att in attrs {
        if let &NameValue(ref name, Lit::Str(ref text, _)) = &att.value {
            if name == "doc" {
                if !ret.is_empty() {
                    ret.push('\n');
                }

                if att.is_sugared_doc {
                    if att.style == syn::AttrStyle::Inner {
                        ret.push_str(text.trim_left_matches("//!"));
                    } else {
                        ret.push_str(text.trim_left_matches("///"));
                    }
                } else {
                    ret.push_str(&text);
                }
            }
        }
    }

    ret
}

fn main() {
    let manifest_root = PathBuf::from(ROOT);
    //TODO: de-hardcode crate root
    let crate_root = manifest_root.join("src/lib.rs");

    //TODO: lol unwrap
    let mod_text = read_all_text(&crate_root).unwrap();
    let krate = syn::parse_crate(&mod_text).unwrap();

    let dox = flatten_docs(&krate.attrs);

    println!("{}", dox);
}
