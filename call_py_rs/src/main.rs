use pyo3::types::{PyString};
use std::{fs, path::Path};
//use pyo3::{PyResult, Python, types::PyList};
use pyembed::OxidizedPythonInterpreterConfig;

include!("../../build/testing/default_python_config.rs");

fn main() {
    let config: OxidizedPythonInterpreterConfig = default_python_config();
    //let current_path =  Path::new("/home/aung/linux_rsoxygen/build/aarch64-unknown-linux-gnu/debug/install/linux_rsoxygen");

    //config.exe = Some(current_path.to_owned());
    //print!("{:?}",config);
    let interpreter = pyembed::MainPythonInterpreter::new(config).unwrap();

    let path = Path::new("/home/aung/linux_rsoxygen/hello");
    let py_app = fs::read_to_string(path.join("main.py")).unwrap();

    interpreter.with_gil(|py| {
        //py.eval(code, globals, locals);
        //let locals = PyString::new(py,"");
        // print!("{:?}",py_app);
        py.run(&py_app, None, None).unwrap();
    });

    // let mut config = OxidizedPythonInterpreterConfig::default();

    // let current_path =  Path::new("/home/aung/linux_rsoxygen/build/aarch64-unknown-linux-gnu/debug/install/linux_rsoxygen");
    // config.origin = Some(current_path.to_owned());
    // config.interpreter_config.base_executable = Some(current_path.to_owned());
    // config.interpreter_config.executable = Some(current_path.to_owned());
    // config.interpreter_config.home = Some(current_path.to_owned());
    // println!("{:?}",config);

    // let interpreter = pyembed::MainPythonInterpreter::new(config).unwrap();

    // Python::with_gil(|py| {
    //     let sys = py.import("sys")?;
    //     let path:&PyList = sys.getattr("path")?.extract()?;
    //     print!("{:?}",path);
    //     path.insert(0, "/home/aung/linux_rsoxygen/build/aarch64-unknown-linux-gnu/debug/install/linux_rsoxygen")?;
    //     print!("{:?}",path);
    //     let my_module = py.import("main")?;
    //     let result = my_module.getattr("main")?;
    //     println!("{:?}",result.call0());
    //     Ok(())
    // })
}

//fn main() {

// let config = OxidizedPythonInterpreterConfig {
//     exe: Some(PathBuf::from("/home/aung/linux_rsoxygen/build/aarch64-unknown-linux-gnu/debug/install/linux_rsoxygen")),
//     ..Default::default()
// };

// let interpreter = pyembed::MainPythonInterpreter::new(config).unwrap();

// interpreter.with_gil(|py| match py.eval("print('hello, world')", None, None) {
//     Ok(_) => print!("python code executed successfully"),
//     Err(e) => print!("python error: {:?}", e),
// });

// let output = Command::new(
//     "/home/aung/linux_rsoxygen/build/aarch64-unknown-linux-gnu/debug/install/linux_rsoxygen",
// )
// .arg("-c")
// .arg("import main; main.main()")
// .output()
// .unwrap();

// println!("{:?}",output);

// let stdout = String::from_utf8_lossy(&output.stdout);
// println!("Python output: {}", stdout);
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
//}
