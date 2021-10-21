use std::fs::{self, File};
use std::io::Write;
use std::{env::args, ffi::OsStr, path::Path};

use koala::grammar::{compiler::CodeGen, parser::parse};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() > 1 {
        let file_path = Path::new(&args[1]);

        let extension = match file_path.extension().and_then(OsStr::to_str) {
            Some(ext) => ext,
            None => return Ok(()),
        };

        if extension == "koala" {
            // match args.iter().any(|arg| arg == "--ast") {
            //     true => {}
            //     false => {}
            // }
            let code = match fs::read_to_string(&args[1]) {
                Ok(file_contents) => file_contents,
                Err(_) => return Ok(()),
            };

            let program = parse(&code);

            let vm_code = program.code_gen();

            let mut output = File::create("test.kvm")?;
            for inst in vm_code {
                if let Err(e) = output.write_all(&inst.to_be_bytes()) {
                    panic!("{}", e);
                }
            }
        }
    }

    Ok(())
}
