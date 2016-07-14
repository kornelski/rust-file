use std::fs::File;
use std::path::Path;
use std::io::{Read, Write, Error};

pub fn get<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error>{
    let mut file = try!(File::open(path));
    let mut data = Vec::new();
    try!(file.read_to_end(&mut data));
    Ok(data)
}

pub fn put<P: AsRef<Path>>(path: P, data: &[u8]) -> Result<(), Error> {
    let mut file = try!(File::create(path));
    try!(file.write_all(data));
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

    std::fs::remove_file(tmp_name).ok();
}
