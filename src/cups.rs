use std::io::Read;
use std::process::Command;
use std::{str, fs};

use crate::ipp::Ipp;

use crate::printer;
use crate::printer::Printer;
use crate::process;


/**
 * Get printers from CUPS server
 */
pub fn get_printers() -> Vec<printer::Printer> {

    // let ipp = Ipp::v2("ipp://localhost:631/printers/_ELGIN_I9".to_string());

    // let operation = ipp.operation.get_printer_attributes();

    // operation.send();

    // return Vec::with_capacity(0);
    return vec![
        printer::Printer::new("ELGIN I9".to_string(), "_ELGIN_I9".to_string(), &self::print),
    ]
}

/**
 * Print on cups usign IPP
 */
pub fn print(printer_system_name: &str, file_path: &str) -> Result<bool, String> {

    println!("{}", printer_system_name.to_string());

    let uri = ["ipp://localhost:631/printers/", printer_system_name].join("");
    
    let ipp = Ipp::v2(uri);

    let data = fs::read(file_path).unwrap();

    let operation = ipp.operation.print_job(
        data,
        None,
        None,
        None
    );

    let result = operation.send();

    if result == false {
        return Result::Err("Print failed".to_string());
    }

    return Result::Ok(true);
}
