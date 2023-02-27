use std::{path::Path};

use pyo3::{Python, types::PyModule};

fn main() {
    println!("Hello, world!");
    let custom_path = Path::new("../../build/aarch64-unknown-linux-gnu/debug/install/linux_rsoxygen");

    println!("{:?}",custom_path);

   
                                

}