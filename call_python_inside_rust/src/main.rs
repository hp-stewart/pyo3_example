use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::types::PyTuple;
use pyo3::types::IntoPyDict;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// starting point

fn main() {
    let _r1 = match simple_inline_python_code() {
        Ok(_) =>     println!("Py Function 1 success!!!\n"),
        Err(_) =>     println!("Py Function 1 failed...\n"),
    };
    let _r2 = match print_python_version() {
        Ok(_) =>     println!("Py Function 2 success!!!\n"),
        Err(_) =>     println!("Py Function 2 failed...\n"),
    };
    let _r3 = match python_function_with_args() {
        Ok(_) =>     println!("Py Function 3 success!!!\n"),
        Err(_) =>     println!("Py Function 3 failed...\n"),
    };
    let _r4 = match python_function_with_kwargs() {
        Ok(_) =>     println!("Py Function 4 success!!!\n"),
        Err(_) =>     println!("Py Function 4 failed...\n"),
    };
    let _r5 = match python_function_from_file() {
        Ok(_) =>     println!("Py Function 5 success!! \n"),
        Err(e) =>     println!("Py Function 5 failed because {e}...\n"),
    };
    let _r6 = match python_function_from_file_updated() {
        Ok(_) =>     println!("Py Function 6 success!! \n"),
        Err(e) =>     println!("Py Function 6 failed because {e}...\n"),
    };

}

// Example functions

fn simple_inline_python_code() -> PyResult<()> {
    // Initialize Python interpreter and acquire Global Interpreter Lock
    Python::with_gil(|py| {
        // python code to evaluate
        let code = "print('Hello, World!')";

        // py.eval is a method to execute a python expression and return the evaluated value as an &PyAny object
        // this method accepts (code: &str, globals: Option<&PyDict>, locals: Option<&PyDict>) and returns PyResult<&'py PyAny>
        // if globals is None, it defauilts to python module __main__
        // If locals is None, it defaults to the value of globals
        // you can optionally use .extract()? to assign the a value to a new variable
        py.eval(code, None, None)?;
        Ok(())
    })
}

fn print_python_version() -> PyResult<()> {
    Python::with_gil(|py| {
        // import and use a python library
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        // another way to perform python imports
        let locals = [("os", py.import("os")?)].into_py_dict(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        
        // run python code using "os" local import)
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;
        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}

fn python_function_with_args() -> PyResult<()> {
    let arg1 = "arg1";
    let arg2 = "arg2";
    let arg3 = "arg3";

    Python::with_gil(|py| {
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            "def example(*args, **kwargs):
                if args != ():
                    print('called with args', args)
                if kwargs != {}:
                    print('called with kwargs', kwargs)
                if args == () and kwargs == {}:
                    print('called with no arguments')",
            "",
            "",
        )?
        .getattr("example")?
        .into();

        // call object without any arguments
        fun.call0(py)?;

        // call object with PyTuple
        let args = PyTuple::new(py, &[arg1, arg2, arg3]);
        fun.call1(py, args)?;

        // pass arguments as rust tuple
        let args = (arg1, arg2, arg3);
        fun.call1(py, args)?;
        Ok(())
    })
}


fn python_function_with_kwargs() -> PyResult<()> {
    let key1 = "key1";
    let val1 = 1;
    let key2 = "key2";
    let val2 = 2;

    Python::with_gil(|py| {
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            "def example(*args, **kwargs):
                if args != ():
                    print('called with args', args)
                if kwargs != {}:
                    print('called with kwargs', kwargs)
                if args == () and kwargs == {}:
                    print('called with no arguments')",
            "",
            "",
        )?
        .getattr("example")?
        .into();

        // call object with PyDict
        let kwargs = [(key1, val1)].into_py_dict(py);
        fun.call(py, (), Some(kwargs))?;

        // pass arguments as Vec
        let kwargs = vec![(key1, val1), (key2, val2)];
        fun.call(py, (), Some(kwargs.into_py_dict(py)))?;

        // pass arguments as HashMap
        let mut kwargs = HashMap::<&str, i32>::new();
        kwargs.insert(key1, 1);
        fun.call(py, (), Some(kwargs.into_py_dict(py)))?;

        Ok(())
    })
}

fn python_function_from_file() -> PyResult<()> {
    // this example based on 
    // https://python.plainenglish.io/using-python-in-rus-and-trust-in-python-ac5cf77d5ece
    //
    Python::with_gil(|py| {
        // first we need to grab the python code from a local file
        // Create a path to the desired file
        let path = Path::new("functions.py");

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file =  File::open(&path).unwrap();

        // Read the file contents into a mutable String object, returns `io::Result<usize>`
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        // create PyModule from contents of file
        // this is used to access individual functions separately
        let functions = PyModule::from_code(
            py,
            &s,
            "functions.py",
            "functions"
        ).unwrap();

        // import and run functions from the module
        let add_function = functions.getattr("add_numbers").unwrap();
        let args = PyTuple::new(py, &[11,23]);
        let function_result = add_function.call1(args).unwrap(); // instead of unwrap, try to handle the pyresult directly?
        println!("\n--------------------------------\nThe sum is {}", function_result);

        //return the string result
        Ok(())
    })
}


fn python_function_from_file_updated() -> PyResult<i32> {
    // this example based on 
    // https://python.plainenglish.io/using-python-in-rus-and-trust-in-python-ac5cf77d5ece
    //
    Python::with_gil(|py| {
        // first we need to grab the python code from a local file
        // Create a path to the desired file
        let path = Path::new("functions.py");

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = File::open(&path).unwrap();

        // Read the file contents into a mutable String object, returns `io::Result<usize>`
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        // create PyModule from contents of file
        // this is used to access individual functions separately
        let functions = PyModule::from_code(
            py,
            &s,
            "functions.py",
            "functions"
        ).unwrap();

        // import and run functions from the module
        let add_function = functions.getattr("add_numbers").unwrap();
        let args = PyTuple::new(py, &[45,33]);
        let function_result:i32 = add_function.call1(args).unwrap().extract()?;
        
        //return the string result
        Ok(function_result)
    })
}