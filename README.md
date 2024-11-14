# IFJ fuzzer
This small primitive fuzzer was created for ifj compiler project to test our compiler for crashes and undefined behavior.
## How to use
The fuzzer accepts 4 arguments `./ifj_fuzz <MODE> <COMPILER> <WORKDIR> <TRY_COUNT>`
- **MODE** there are 3 modes `ascii`, `bytes`, `tokens`
- - ascii - will generate valid ascii characters (some of them may not be visible)
- - bytes - will generate random bytes that may or may not match any praticular characters
- - tokens - will generate random tokens some strings may not be valid (the code should not compile to valid ifjcode)
- **COMPILER** this argument specifies location of compiler executable ie. `./compiler`
- **WORKDIR** this is directory where all failcases & currently executed code will be stored the format should be `path/to/directory/`. Please use empty directory.
- **TRY_COUNT** how many files should be generated and tested for example `50`
