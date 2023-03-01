# tests directory
- Cargo knows to look for integration test files in this directory. 
- Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test
- Each file in the test directory is compiled into a separate crate.
