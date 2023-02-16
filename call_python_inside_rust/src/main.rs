use examples;
fn main() {
    // for troubleshooting--displays local python packages
    examples::display_package_info();
    println!("\n----------Begin PYO3 example functions -------------------------------------\n");
    // each of the following examples shows a different way to call Python from Rust
    /*
    println!("\nExample 1: simple inline code");
    let _r1 = match examples::simple_inline_python_code() {
        Ok(_) =>     println!("\nPy Function 1 success!!!\n"),
        Err(_) =>     println!("\nPy Function 1 failed...\n"),
    };
    println!("\nEnd\n--------------------------------------------------\n");

    println!("\nExample 2: inline code + library import");
    let _r2 = match examples::print_python_version() {
        Ok(_) =>     println!("\nPy Function 2 success!!!\n"),
        Err(_) =>     println!("\nPy Function 2 failed...\n"),
    };
    println!("\nEnd\n--------------------------------------------------\n");

    println!("\nExample 3:  No args, vs PyTuple args, vs rust tuple args");
    let _r3 = match examples::python_function_with_args() {
        Ok(_) =>     println!("\nPy Function 3 success!!!\n"),
        Err(_) =>     println!("\nPy Function 3 failed...\n"),
    };
    println!("\nEnd\n--------------------------------------------------\n");
    
    println!("\nExample 4: kwargs as PyDict, Vec, or Hashmap");
    let _r4 = match examples::python_function_with_kwargs() {
        Ok(_) =>     println!("\nPy Function 4 success!!!\n"),
        Err(_) =>     println!("\nPy Function 4 failed...\n"),
    };
    println!("\nEnd\n--------------------------------------------------\n");

    println!("\nExample 5: call from local .py file");
    let _r5 = match examples::python_function_from_file() {
        Ok(n) =>     println!("\nPy Function 5 success!! \nThe result was {n:?} \n"),
        Err(e) =>     println!("\nPy Function 5 failed because {e}...\n"),
    };
    println!("\nEnd\n--------------------------------------------------\n");

    println!("\nExample 6: error handling");
    let _r6 = match examples::python_function_err_handling() {
        Ok(n) =>     println!("\nPy Function 6 success!! \nThe result was {n:?} \n"),
        Err(e) =>     println!("\nPy Function 6 failed because {e}...\n"),
    };
    println!("\nEnd\n--------------------------------------------------\n");
     */

    println!("\nExample 7a: functions requiring packages installed on venv -- returns PyResult<i32, PyErr>");
    let _r7a = match examples::python_function_venv_a() {
        Ok(n) =>     println!("\nPy Function 7a success!! \nThe result was {n:?} \n"),
        Err(pyerr) =>     println!("\nPy Function 7a failed because {pyerr} ...\n"),
    };
    println!("\n\n---------------------------------------------------------------------------\n---------------------------------------------------------------------------\n");

    println!("\nExample 7b: functions requiring packages installed on venv -- returns Result<i32, Error>");
    let _r7b = match examples::python_function_venv_b() {
        Ok(n) =>     println!("\nPy Function 7b success!! \nThe result was {n:?} \n"),
        Err(e) =>     println!("\nPy Function 7b failed because {e}...\n"),
    };
    println!("\n\n---------------------------------------------------------------------------\n---------------------------------------------------------------------------\n");

    println!("\nExample 7c: functions requiring packages installed on venv -- returns Result<Option<char>, Error>");
    let _r7c = match examples::python_function_venv_c("red") {
        Ok(n) =>     println!("\nPy Function 7c success!! \nThe result was Ok({n:?}) \n"),
        Err(e) =>     println!("\nPy Function 7c failed because {e}...\n"),
    };

    println!("\nEnd\n--------------------------------------------------\n");

}



