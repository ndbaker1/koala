# Koala ʕ •ᴥ•ʔ
A simple programming language packaged with a virtual machine created for learning purposes.  
Try out the demo site [here 🌍](https://ndbaker1.github.io/koala/)


###### *THIS LANGUAGE IS NOT MEANT TO BE USABLE, IT IS JUST FOR GIVING MYSELF AND OTHERS A FLEXIBLE VIEW OF HOW A MACHINE RUNS HIGH-LEVEL CODE*

## Inspiration
**Koala** was created as a programming/design exercise while attending the University of Texas at Dallas.

In order to get a better understanding of how a high level programming language executes on a machine I decided to make a small and portable language. 

Koala is written in `Rust`, making it a great candidate for cross-platform deployment, and also gets the benefit of being able to compile to `WebAssembly` by using `wasm-pack`  

The VM Runtime principle was inspired by languages like `Java` and `C#`.
The **Java Virtual Machine (JVM)**, and **.NET Core** are stack machines, which often end up executing many more instructions than register-based machine, but are much simpler to generate code for. 

Phases of the Project:
  1. Implement a simple Virtual Machine (stack-based)
  2. Design a small yet capable language grammar (procedural, arrays, recursion)
  3. Parse source code into an Abstract Sytax Tree
  4. Perform Code Generation for an AST, targetting our Virtual Machine platform
  5. Create regression tests so that it is clear when changes to any of the previous components is a breaking change (Testing is Important!)

## Development
Build everything in the Rust project before testing the binaries `koala` and `koalac`
```sh
cargo build
...
cargo run --bin koala
cargo run --bin koalac
```
When testing the browser UI, package the wasm using `wasm-pack` or use the provided `koala-build.sh` script
```sh
# root project directory
./koala-build.sh
```
Startup the browser UI using
```sh
npm run dev
```

# Extra info

## Compiler Architecture
A typical Compiler architecture consists of 3 core analyzers:  
1. Lexical Analyzer
2. Syntactic Analyzer
> Koala uses the PEG parser in order to parse source code into `Rust` objects, which is performing lexical and syntactic analysis for us. Parsing Expression Grammars (PEGs) are much simpler to write rules for than convenional LL(k) grammars because recursive descent parsing approaches simulate infinite lookahead
3. Semantic Analyzer

> Koala does not contain a semantic anaylzer portion, since it would have taken more development time than I wanted for a short project. If I was able to implement the semantic analyzer then I would have been able to provided type checking at compile time, syntax suggestions/error catching, and more features

and is split into 3 groups:
1. Front End
> Essentially consists of all the tasks involved to transform source code into an easily manipulatable form.  
2. Middle End
> May be in an Intermediate Representation (IR) such as Kotlin IR or LLVM, which can undergo several cycles of optimization.  
3. Back End
> Code Generation takes place in this step, which is a platform specific process and can consist of modules for many different targets

## Self Hosted Compilers
The concept of a self-hosted compiler is interesting; that is, a compiler which can compile the compiler for its own language.
The bootstrapping process is used to characterize the stages needed to reach a self-hosted compiler.

#### Stage 1
You must begin with a compiler in another start language.  
#### Stage 2
Once the language is stable enough to write implement its own compiler, write and compile the compiler.
#### Stage 3
Using the newely generated compiler, compile the source code for the compiler once again. If this compiler binary is no different than the compiler from the previous stage, then the compiler is self-sufficient.