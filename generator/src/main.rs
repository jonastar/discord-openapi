use std::{fmt::Write, io::Write as _};

use spec::Spec;

mod code_gen;
// mod generated;
mod ir;
mod rust_codegen;
mod spec;

const SPEC_STR: &str = include_str!("spec.json");

const HEADER: &'static str = "use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

use crate::{IntTagVisitor, StringTagVisitor};
";

fn main() {
    let spec: Spec = serde_json::from_str(SPEC_STR).unwrap();
    // dbg!(spec);

    println!("Running");
    let ir = ir::generate_ir_from_spec(&spec);

    let rust_generator = rust_codegen::RustCodeGen {};
    let code_output = code_gen::generate_code(ir, rust_generator);

    let mut file = std::fs::File::create("../generated-rust/src/generated.rs").unwrap();

    file.write_all(HEADER.as_bytes()).unwrap();
    file.write_all(code_output.as_bytes()).unwrap();
    file.flush().unwrap();
    println!("Done!");
}

pub fn maybe_rename_camel_case(input: &str) -> Option<String> {
    let mut buf = String::new();

    let mut next_upper = true;
    for char in input.chars() {
        if char == '_' || char == ' ' || char == '-' {
            next_upper = true;
            continue;
        }

        if next_upper {
            next_upper = false;
            let upper_chars = char.to_uppercase();
            for upper in upper_chars {
                buf.write_char(upper).unwrap();
            }
        } else {
            let lower_chars = char.to_lowercase();
            for lower in lower_chars {
                buf.write_char(lower).unwrap();
            }
        }
    }

    if &buf == input {
        None
    } else {
        Some(buf)
    }
}

pub fn maybe_rename_camel_case_keep_case(input: &str) -> Option<String> {
    let mut buf = String::new();

    let mut next_upper = true;
    for char in input.chars() {
        if char == '_' || char == ' ' || char == '-' {
            next_upper = true;
            continue;
        }

        if next_upper {
            next_upper = false;
            let upper_chars = char.to_uppercase();
            for upper in upper_chars {
                buf.write_char(upper).unwrap();
            }
        } else {
            buf.write_char(char).unwrap();
            // let lower_chars = char.to_lowercase();
            // for lower in lower_chars {
            //     buf.write_char(lower).unwrap();
            // }
        }
    }

    if &buf == input {
        None
    } else {
        Some(buf)
    }
}
