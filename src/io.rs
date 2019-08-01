use std::{
    fs::{
        File,
    },
    io::{
        BufRead,
        BufReader,
    },
};

pub fn open_file(
    file_name: &str,
) -> Vec<String> {
    BufReader::new(
        File::open(file_name).expect("error opening file"),
    ).lines().map(|line_result| {
        line_result.expect("error reading line")
    }).collect()
}
