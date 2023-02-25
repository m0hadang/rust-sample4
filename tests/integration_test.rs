extern crate elf_reader;

use elf_reader::elf_reader::add_two;

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}