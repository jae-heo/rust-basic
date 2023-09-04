mod practice_panic;

fn main() {
    match practice_panic::test_result() {
        Ok(_) => println!("Read file without problem!"),
        Err(err) => println!("Read file without problem! Error : {:?}", err),
    }
}

/*
Modulization is in order of...
Modules < Crates < Package

Module is the smallest part of Rust program
Multiple modules can be defined from crate root, by creating mod in the root file or in different files.

As Modules are included in root, the crate contains the module.

The crate has two kinds, binary/library

Still confusing how should I divide program into modules, crates and packages.

As our main function does not change even if we divide modules into saperated files.
This means that we can dynamically chagne the file structure without change the paths (because it depends on module tree)
*/