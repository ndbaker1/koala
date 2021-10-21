use std::fs::{self, File};
use std::io::Write;
use std::{env::args, ffi::OsStr, path::Path};

use koala::grammar::compiler::CodeGen;
use koala::grammar::parser::parse_code;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() > 1 {
        let file_path = Path::new(&args[1]);

        let extension = match file_path.extension().and_then(OsStr::to_str) {
            Some(ext) => ext,
            None => return Ok(()),
        };

        if extension == "koala" {
            let file_string = match fs::read_to_string(&args[1]) {
                Ok(file_contents) => file_contents,
                Err(_) => return Ok(()),
            };

            let program = match parse_code(&file_string) {
                Ok(prog) => prog,
                Err(e) => panic!("{}", e),
            };

            if args.iter().any(|arg| arg == "--ast") {
                let ast = serde_json::to_string_pretty(&program)?;
                fs::write("test.kast", ast)?;
            } else {
                let vm_code = program.code_gen();

                let mut output = File::create("test.kvm")?;
                for inst in vm_code {
                    output.write_all(&inst.to_be_bytes())?;
                }
            }
        } else {
            eprintln!("ʕ•ᴥ• ʔ {:?} wasnt a .koala", file_path);
        }
    } else {
        eprintln!("ʕ •ᴥ•ʔ why no file?");
    }

    Ok(())
}
