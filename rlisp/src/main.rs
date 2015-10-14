#![feature(custom_derive)]
#![feature(convert)]
#![feature(split_first)]
#![feature(clone_from_slice)]
#![feature(slice_splits)]
extern crate clap;
mod parser;
mod data;
mod eval;
mod stdlisp;

use clap::{Arg, App};
use data::*;
use parser::*;
use eval::*;

fn main() {
    let matches = App::new("rlisp")
        .version("1.0")
        .author("Edward Yang <edward.yang6771@gmail.com>")
        .about("Basic lisp interpreter in rust")
        .args_from_usage(
            "-i --interactive=[INTERACTIVE] 'optional - Enables interactive repl - enabled if no file specified'
            -f --file=[FILE] 'optional - specifies a file to load'")
        .get_matches();
    //let bool interactive;
    //if let Some(input) = matches.value_of("FILE") {
        
    //} else {
    
    //}
    let parsed = parse(&"(cons 1 (list 1 2))".to_string());
    match parsed {
        Expr::Exprs(ref xs) => {println!("{:#?}", xs)},
        Expr::Expr(ref x) => {println!("{:#?}", x)},
    }
    let mut stdenv = Env::new();
    println!("{:#?}", parsed.eval(&mut stdenv));
}

fn repl(file: &str) {
    unimplemented!();
}
