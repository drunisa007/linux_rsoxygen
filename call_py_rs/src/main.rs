use std::process::Command;


fn main() {
    let output = Command::new(
        "/home/aung/linux_rsoxygen/build/aarch64-unknown-linux-gnu/debug/install/linux_rsoxygen",
    )
    .arg("-c")
    .arg("import main; main.main()")
    .output()
    .unwrap();

    println!("{:?}",output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Python output: {}", stdout);
    // let mut config = OxidizedPythonInterpreterConfig::default();
    // config.interpreter_config.parse_argv = Some(false);
    // config.set_missing_path_configuration = false;
    // config.interpreter_config.executable = Some("/home/aung/linux_rsoxygen/build/aarch64-unknown-linux-gnu/debug/install/linux_rsoxygen".into());

    // let intepreter = MainPythonInterpreter::new(config).unwrap();
    // intepreter.with_gil(|py| {
    //    py.eval("print('hello world')",None,None).unwrap();
    //    let module = py.import("main").unwrap();
    //    let main_func = module.getattr("main").unwrap();
    //    main_func.call0().unwrap();

    // });

    //println!("Hello, world!");
    // let custom_path = Path::new(
    //     "/home/aung/linux_rsoxygen/build/aarch64-unknown-linux-gnu/debug/install/linux_rsoxygen",
    // );

    // const PYTHON_EXECUTABLE: &'static [u8] = include_bytes!(
    //     "/home/aung/linux_rsoxygen/build/aarch64-unknown-linux-gnu/debug/install/linux_rsoxygen"
    // );

    // let code = std::str::from_utf8(PYTHON_EXECUTABLE).unwrap();

    // //println!("{:?}",custom_path);

    // const PYTHON_EXECUTABLE: &'static [u8] = include_bytes!("../../build/aarch64-unknown-linux-gnu/debug/install/linux_rsoxygen");

    // let module_name = "hello";
    // //let script_file = "main.py";
    // let code = std::str::from_utf8(PYTHON_EXECUTABLE).unwrap();
}
