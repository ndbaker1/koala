# Koala ʕ •ᴥ•ʔ
A simple programming language packaged with a virtual machine created for learning purposes.  
Try out the demo site [here](https://ndbaker1.github.io/koala/)
## Inspiration
Koala was inspired by languages like `Java` and `C#`, which utilize a virtual machine to facilitate a cross-platform runtime.
The **Java Virtual Machine (JVM)**, and **.NET Core** are stack machines, which often end up executing many more instructions than register-based machine, but are much simpler to generate code for. 

## Development
Koala is written in `Rust`, making it a great candidate for cross-platform deployment, and also gets the benefit of being able to compile to **WebAssembly** by using `wasm-pack`  

Build everything in the Rust project before testing the binaries `koala` and `koalac`
```sh
cargo build
...
cargo run --bin koala
cargo run --bin koalac
```
When testing the browser UI, first package the wasm using `wasm-pack` or default to the `koala-build.sh` script
```sh
# root project directory
./koala-build.sh
```
Startup the browser UI using
```sh
npm run dev
```
