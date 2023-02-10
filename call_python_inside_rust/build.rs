use std::fs;

use pyo3_build_config::{BuildFlags, PythonVersion, PythonImplementation};

fn main() {

    println!("Running build.rs...");

    let pyo3_interpreter = &_configure_py_for_venv();
    println!("\nPython Interpreter: {:?}\n{:?}\n", 
        &pyo3_interpreter.lib_name.to_owned(),
        &pyo3_interpreter.lib_dir.to_owned(),
    );

    let output = &pyo3_interpreter
        .run_python_script(
            &fs::read_to_string("./py/functions_venv.py")
                .expect("failed to read functions.py"),
        )
        .expect("failed to run functions.py");
    println!("{}", output);

    println!("cargo:PYO3_CONFIG=/home/hstewart/repos/pyo3_example/call_python_inside_rust/pyo3-build-config.txt");
    println!("cargo:rerun-if-changed=build.r,");
}


fn _configure_py_for_venv() -> pyo3_build_config::InterpreterConfig {
    pyo3_build_config::InterpreterConfig{
        implementation: PythonImplementation::CPython,
        version: PythonVersion{major:3, minor:10},
        shared: true,
        abi3: false,
        lib_name:Some(String::from("python3.10")),
        lib_dir:Some(String::from("/home/hstewart/repos/pyo3_example/call_python_inside_rust/env/lib")),
        executable:Some(String::from("/home/hstewart/repos/pyo3_example/call_python_inside_rust/env/bin/python3.10")),
        pointer_width: Some(64),
        build_flags: BuildFlags::default(),
        suppress_build_script_link_lines: false,
        extra_build_script_lines: vec![],
    }
}