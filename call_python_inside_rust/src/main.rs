use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;

use std::process::Command; // used to do "$pip list" from inside Rust

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::types::PyTuple;
use pyo3::types::IntoPyDict;


// starting point

fn main() {
    // for troubleshooting--displays local python packages
    display_package_info();
    println!("\n----------Begin PYO3 example functions -------------------------------------\n");
    // each of the following examples shows a different way to call Python from Rust
    println!("\nExample 1: simple inline code");
    let _r1 = match simple_inline_python_code() {
        Ok(_) =>     println!("Py Function 1 success!!!\n"),
        Err(_) =>     println!("Py Function 1 failed...\n"),
    };
    println!("\nEnd\n--------------------------------------------------\n");

    println!("\nExample 2: inline code + library import");
    let _r2 = match print_python_version() {
        Ok(_) =>     println!("Py Function 2 success!!!\n"),
        Err(_) =>     println!("Py Function 2 failed...\n"),
    };
    println!("\nEnd\n--------------------------------------------------\n");

    println!("\nExample 3:  No args, vs PyTuple args, vs rust tuple args");
    let _r3 = match python_function_with_args() {
        Ok(_) =>     println!("Py Function 3 success!!!\n"),
        Err(_) =>     println!("Py Function 3 failed...\n"),
    };
    println!("\nEnd\n--------------------------------------------------\n");
    
    println!("\nExample 4: kwargs as PyDict, Vec, or Hashmap");
    let _r4 = match python_function_with_kwargs() {
        Ok(_) =>     println!("Py Function 4 success!!!\n"),
        Err(_) =>     println!("Py Function 4 failed...\n"),
    };
    println!("\nEnd\n--------------------------------------------------\n");

    println!("\nExample 5: call from local .py file");
    let _r5 = match python_function_from_file() {
        Ok(n) =>     println!("Py Function 5 success!! \nThe result was {n:?} \n"),
        Err(e) =>     println!("Py Function 5 failed because {e}...\n"),
    };
    println!("\nEnd\n--------------------------------------------------\n");

    println!("\nExample 6: error handling");
    let _r5 = match python_function_err_handling() {
        Ok(n) =>     println!("Py Function 6 success!! \nThe result was {n:?} \n"),
        Err(e) =>     println!("Py Function 6 failed because {e}...\n"),
    };
    println!("\nEnd\n--------------------------------------------------\n");

}

// Misc Helper functions

fn display_package_info() {
    println!("\nRunning PIP to see packages...");

    println!("\nGlobal Packages:");
    let output = Command::new("bash")
        .args(["-c","pip3 list"])
        .output()
        .expect("bash command failed");
    println!("Pip list \tstatus: {:?}",output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();


    println!("\nLocal Packages:");
    let output = Command::new("bash")
        .arg("-c")
        .arg("pip3 freeze --local")
        .output()
        .expect("bash command failed");
    println!("pip freeze --local:\tStatus: {:?}",output.status);
    match output.stdout.len() {
        0 =>println!("No virtual environment detected\nNo packages to display"),
        _=> {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }
    }

}

fn get_py_file_contents(file_name:&str) -> String {
    // first we need to grab the python code from a local file
    // Create a path to the desired file
    let path = Path::new(file_name);

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = File::open(&path).unwrap();

    // Read the file contents into a mutable String object, returns `io::Result<usize>`
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    return s
}

// Example functions are defined below

// Example 1
// Simplest method-- write a python snippet as a rust string and execute it using .eval()
// The function returns a PyResult which is a public type representing the result of a python call
//    pub type PyResult<T> = Result<T, PyErr>
// where PyErr  represents a python exception 
fn simple_inline_python_code() -> PyResult<()> {
    // Initialize Python interpreter and acquire Global Interpreter Lock
    println!("\nInitializing py interpreter...");
    Python::with_gil(|py| {
        // define the python code to be evaluated
        let code = "print('Hello, World!')";
        println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code:?}\n\n-----end of py code-----");

        // execute the code using .eval()
        // the method returns the result evaluated from the expression as an &PyAny object
        // this method accepts (code: &str, globals: Option<&PyDict>, locals: Option<&PyDict>) and returns PyResult<&'py PyAny>
        // if globals is None, it defauilts to python module __main__
        // If locals is None, it defaults to the value of globals
        // you can optionally use .extract()? to assign the a value to a new variable
        println!("\nEvaluating...\n-----start of py output-----\n");
        py.eval(code, None, None)?;
        println!("\n-----end of py output-----\nEvaluation completed\n");

        // Finished--> PyResult = Ok
        Ok(())
    })
}


// Example 2
// This example shows two ways to import a package 
// that has been installed for the active python interpreter
// the python snippet in this example depends on import sys and 
// import os, two default packages
fn print_python_version() -> PyResult<()> {
    // Initialize Python interpreter and acquire Global Interpreter Lock
    println!("\nInitializing py interpreter...");
    Python::with_gil(|py| {
        // You can get certain data from py modules without writing any inline code 
        // The first example will retrieve the current version of python from the "sys" module
        
        // import the library to the active interpreter session
        // .getattr() can be used to retrieve an attribute vale, in this case "version"
        //     self.getattr(attr_name) 
        // in Rust is equivalent to the python expression 
        //     self.attr_name
        // .extract() is used to get the corresponding rust type binding from a python object 
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        // The second example will add an imported library to some inline python code
        // import the library or libraries to be used
        // create a vector of tuples representing each library with a name & pointer
        // then convert the vector to a python dictionary
        let os = py.import("os")?; 
        let locals = [("os", os)].into_py_dict(py);
        
        // define the code to be executed
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code}\n\n-----end of py code-----");

        // evaluate the python expression using .eval()
        // remember to pass in the libraries via "locals"
        // and convert the result from a python type to rust type using .extract()
        println!("\nEvaluating...\n-----start of py output-----\n");
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;
        println!("\n-----end of py output-----\nEvaluation completed");

        // the values we got from python can now be used in Rust
        println!("\nRust Output:\n\tUser: {}, \n\tVersion: {}\n", user, version);

        // Finished--> PyResult = Ok
        Ok(())
    })
}

// Example 3
// This example shows some ways to pass one or more args from Rust to Python
// It also shows how to evaluate a python code containing multiple expressions
fn python_function_with_args() -> PyResult<()> {
    // Define the arguments that will be sent to the python function
    let arg1 = "arg1";
    let arg2 = "arg2";
    let arg3 = "arg3";

    // Initialize Python interpreter and acquire Global Interpreter Lock
    println!("\nInitializing py interpreter...");
    Python::with_gil(|py| {
        // Previous examples executed a single expression using .eval()
        // this time a longer snippet will be created--a function definition
        let code = "def example(*args, **kwargs):
        if args != ():
            print('called with args', args)
        if kwargs != {}:
            print('called with kwargs', kwargs)
        if args == () and kwargs == {}:
            print('called with no arguments')";
        println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code}\n\n-----end of py code-----");

        // for longer code we can create a pyModule instead of using .eval()
        // it can contain one or more function definitions
        // the functions can then be accessed as attributes of the pymodule
        let example_function: Py<PyAny> = PyModule::from_code(
            py,
            &code,
            "",
            "",
        )? // create a PyModule from the provided snippet of code
        .getattr("example")? // extract the function called "example"
        .into(); // type conversion

        // Now we can use the python function "example" using the example_function object 
        // To call the function without any arguments use .call0()
        println!("\nDemo#3.1 Call function without arguments\nEvaluating...\n-----start of py output-----\n");
        example_function.call0(py)?;
        println!("\n-----end of py output-----\nDemo#3.1 completed\n");

        // args can be provided via PyTuple
        // To call the function with one or more arguments, use .call1()
        println!("\nDemo#3.2 Call function with PyTuple arguments\nEvaluating...\n-----start of py output-----\n");
        let args = PyTuple::new(py, &[arg1, arg2, arg3]);
        example_function.call1(py, args)?;
        println!("\n-----end of py output-----\nDemo#3.2 completed\n");

        // args can also be provided via Rust Tuple
        // To call the function with one or more arguments, use .call1()
        println!("\nDemo#3.3 Call function with Rust Tuple arguments\nEvaluating...\n-----start of py output-----\n");
        let args = (arg1, arg2, arg3);
        example_function.call1(py, args)?;
        println!("\n-----end of py output-----\nDemo#3.3 completed\n");

        // Finished--> PyResult = Ok
        Ok(())
    })
}

// Example 4
// This example shows some ways to pass one or more kwargs from Rust to Python
fn python_function_with_kwargs() -> PyResult<()> {
    // Define the keywords arguments that will be sent to the python function
    let key1 = "key1";
    let val1 = 1;
    let key2 = "key2";
    let val2 = 2;
    
    // Initialize Python interpreter and acquire Global Interpreter Lock
    println!("\nInitializing py interpreter...");
    Python::with_gil(|py| {
        // define the python code to be run
        let code = "def example(*args, **kwargs):
        if args != ():
            print('called with args', args)
        if kwargs != {}:
            print('called with kwargs', kwargs)
        if args == () and kwargs == {}:
            print('called with no arguments')";
        println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code}\n\n-----end of py code-----");
        
        // create a new PyModule containing the python code and extract the function using .getattr()
        let example_function: Py<PyAny> = PyModule::from_code(
            py,
            code,
            "",
            "",
        )?
        .getattr("example")?
        .into();

        // pass kwargs with PyDict
        println!("\nDemo#4.1 Call function with kwargs from PyDict\nEvaluating...\n-----start of py output-----\n");
        let kwargs = [(key1, val1)].into_py_dict(py);
        example_function.call(py, (), Some(kwargs))?;        
        println!("\n-----end of py output-----\nDemo#4.1 completed\n");


        // pass kwargs with Vec
        println!("\nDemo#4.2 Call function with kwargs from vec!\nEvaluating...\n-----start of py output-----\n");
        let kwargs = vec![(key1, val1), (key2, val2)];
        example_function.call(py, (), Some(kwargs.into_py_dict(py)))?;
        println!("\n-----end of py output-----\nDemo#4.2 completed\n");

        // pass arguments as HashMap
        println!("\nDemo#4.3 Call function with kwargs from HashMap\nEvaluating...\n-----start of py output-----\n");
        let mut kwargs = HashMap::<&str, i32>::new();
        kwargs.insert(key1, 1);
        example_function.call(py, (), Some(kwargs.into_py_dict(py)))?;
        println!("\n-----end of py output-----\nDemo#4.3 completed\n");

        Ok(())
    })
}

// Example 5
// In this example the python code is provided by a separate .py file
// the contents of the file are read and the functions defined inside
// can be used similar to previous examples
// This example based in part on 
// https://python.plainenglish.io/using-python-in-rus-and-trust-in-python-ac5cf77d5ece
fn python_function_from_file() -> PyResult<i32> {
    
    // Initialize Python interpreter and acquire Global Interpreter Lock
    println!("\nInitializing py interpreter...");
    Python::with_gil(|py| {
        // first we need to grab the python code from a local file
        // Create a path to the desired file
        let code = get_py_file_contents("functions.py"); 
        println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code}\n\n-----end of py code-----");
        
        // create PyModule from contents of file
        // this is used to access individual functions separately
        let functions = PyModule::from_code(
            py,
            &code,
            "functions.py",
            "functions"
        ).unwrap();

        // grab the desired function using .getattr("function_name")
        // call the function using .call0() exe without args (or in this case use default args)
        println!("\nDemo#5.1 Call function without args\nEvaluating...\n-----start of py output-----\n");
        let add_function = functions.getattr("add_numbers").unwrap();
        let function_result = add_function.call0().unwrap(); // instead of unwrap, try to handle the pyresult directly?
        println!("\n-----end of py output-----\n");
        // the values we got from python can now be used in Rust
        println!("\nRust Output:\n\tThe sum is {}", function_result);

        // grab the desired function using .getattr("function_name")
        // then create some args and call the function using .call1(args)
        println!("\nDemo#5.2 Call function with PyTuple args\nEvaluating...\n-----start of py output-----\n");
        let add_function = functions.getattr("add_numbers").unwrap();
        let args = PyTuple::new(py, &[11,23]);
        let function_result = add_function.call1(args).unwrap().extract()?; // instead of unwrap, try to handle the pyresult directly?
        println!("\n-----end of py output-----\n");
        // the values we got from python can now be used in Rust
        println!("\nRust Output:\n\tThe sum is {}", function_result);

        //return the string result
        Ok(function_result)
    })
}


// Example 6
// 
fn python_function_err_handling()-> PyResult<i32> {
    
    // Initialize Python interpreter and acquire Global Interpreter Lock
    println!("\nInitializing py interpreter...");
    Python::with_gil(|py| {
        // first we need to grab the python code from a local file
        // Create a path to the desired file
        let code = get_py_file_contents("functions.py"); 
        println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code}\n\n-----end of py code-----");
        
        // create PyModule from contents of file
        // this is used to access individual functions separately
        let functions = PyModule::from_code(
            py,
            &code,
            "functions.py",
            "functions"
        ).unwrap();

        // wrong type (float) args
        println!("\nDemo#6.1 Call function with wrong type args(f32)\nEvaluating...\n-----start of py output-----\n");
        let add_function = functions.getattr("add_numbers").unwrap();
        let args = PyTuple::new(py, &['a','b']);
        let function_result = add_function.call1(args).unwrap().extract()?; // instead of unwrap, try to handle the pyresult directly?
        println!("\n-----end of py output-----\n");
        

        // the values we got from python can now be used in Rust
        println!("\nRust Output:\n\tThe sum is {}", function_result);

        //return the result
        Ok(function_result)
    })
}
