use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");

    let path = PathBuf::from("path/to/file.txt");
    let mut file = File::create(path).unwrap();

    for (k, v) in &map {
        writeln!(file, "{}: {}", k, v).unwrap();
    }
}
