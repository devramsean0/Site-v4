pub fn debug(message: &str, verbose: bool) {
    if verbose {
        println!("DEBUG: {message}");
    }
}

pub fn info(message: String) {
    println!("INFO: {message}");
}
