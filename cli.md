```
lemonc - lemon language compiler

USAGE:
  lemonc <command> [input_file] [options]

COMMANDS:
  check     verify code without generating a binary
  run       execute code in dev mode (hybrid compile-time, no heavy optimizations)
  build     produce a final optimized binary

OPTIONS:
  -o <file>                           output file path
  --backend <llvm|cranelift>          select codegen backend
  --opt-level <none|speed|size|max>   set optimization level
  --linker <mold|lld|msvc>            select linker
  --no-std                            disable standard library
  --target <triple>                   set compilation target (e.g., x86_64-unknown-linux-gnu)
  --verbose                           enable detailed messages

DEBUG/DEV:
  --check-llvm-ir                     verify LLVM IR
  --debug-type                        show internal type IDs
  --emit <ast,hir,mir,llvmir>         emit ir
  --print <tokens,span,types>         inspect internal compiler structures

EXAMPLES:
  lemonc check main.ln --verbose
  lemonc run main.ln --opt-level=speed --backend llvm --debug-type
  lemonc build main.ln -o main.o --backend llvm --opt-level=max --linker mold
```
