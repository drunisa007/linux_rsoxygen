use std::path::Path;


fn main() {
    //println!("Hello, world!");
    // let custom_path = Path::new(
    //     "/home/aung/linux_rsoxygen/build/aarch64-unknown-linux-gnu/debug/install/linux_rsoxygen",
    // );

    const PYTHON_EXECUTABLE: &'static [u8] = include_bytes!(
        "/home/aung/linux_rsoxygen/build/aarch64-unknown-linux-gnu/debug/install/linux_rsoxygen"
    );

    let code = std::str::from_utf8(PYTHON_EXECUTABLE).unwrap();

    // //println!("{:?}",custom_path);

    // const PYTHON_EXECUTABLE: &'static [u8] = include_bytes!("../../build/aarch64-unknown-linux-gnu/debug/install/linux_rsoxygen");

    // let module_name = "hello";
    // //let script_file = "main.py";
    // let code = std::str::from_utf8(PYTHON_EXECUTABLE).unwrap();
}
