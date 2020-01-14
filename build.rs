fn main() {
    // Generate files used by static reload tests
    std::fs::create_dir_all("tests/temp").unwrap();
    std::fs::write("tests/temp/static_changed.txt", "Old").unwrap();
    std::fs::write("tests/temp/static_reload.txt", "Old").unwrap();
    std::fs::write("tests/temp/static_reload_if_changed.txt", "Old").unwrap();
}
