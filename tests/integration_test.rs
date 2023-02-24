extern crate elf_reader;

#[test]
fn it_adds_two() {
    assert_eq!(4, elf_reader::add_two(2));
}