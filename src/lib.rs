use std::fs::File;
use std::path::Path;
use std::io;
use std::io::{Read, Write};

/// Read a file into `Vec<u8>` from the given path.
/// The path can be a string or a `Path`.
pub fn get<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    if let Ok(meta) = file.metadata() {
        data.reserve(meta.len() as usize); // Safe to truncate, since it's only a suggestion
    }
    file.read_to_end(&mut data)?;
    Ok(data)
}

/// Creates a file at the given path with contents of `Vec<u8>` or `&[u8]`, etc.
/// Overwrites, non-atomically, if the file exists.
/// The path can be a string or a `Path`.
pub fn put<P: AsRef<Path>, Bytes: AsRef<[u8]>>(path: P, data: Bytes) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data.as_ref())?;
    Ok(())
}

#[test]
fn it_works() {
    let s = String::from_utf8(get(file!()).unwrap()).unwrap();
    assert!(s.contains("it_works()"));

    let mut tmp_name = std::env::temp_dir();
    tmp_name.push("tmp_file_should_not_exist");

    assert!(get(tmp_name).is_err());

    let mut tmp_name = std::env::temp_dir();
    tmp_name.push("tmp_file_created");

    let data = vec![0u8,1,2,3];
    put(&tmp_name, &data).unwrap();
    assert_eq!(data, get(&tmp_name).unwrap());
    put(&tmp_name, data).unwrap();

    std::fs::remove_file(tmp_name).ok();
}
