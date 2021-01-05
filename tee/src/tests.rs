use super::*;
use assert_cmd::Command;
use std::{error::Error, fs::File};
use tempfile::NamedTempFile;

#[test]
fn tee_copy_buffer() {
    let buffer = b"foo";
    let mut out = Vec::new();

    copy_buffer(BufReader::new(&buffer[..]), &mut out).unwrap();

    assert_eq!(String::from_utf8(out).unwrap(), "foo".to_string());
}

#[test]
fn tee_copy_stdin_to_stdout() -> Result<(), Box<dyn Error>> {
    let buffer = "Hello World!";

    let mut cmd = Command::cargo_bin("tee")?;
    cmd.write_stdin(buffer).assert().stdout(buffer);

    Ok(())
}

#[test]
fn tee_copy_stdin_to_file() -> Result<(), Box<dyn Error>> {
    let buffer = "Hello World!";
    let temp_file = NamedTempFile::new()?;

    let mut cmd = Command::cargo_bin("tee")?;
    cmd.arg("-a").arg(temp_file.path()).write_stdin(buffer).output()?;

    let mut file = File::open(temp_file)?;
    let mut file_buffer = String::new();
    file.read_to_string(&mut file_buffer)?;

    assert_eq!(buffer.to_owned(), file_buffer);

    Ok(())
}

#[test]
fn tee_append_stdin_to_file() -> Result<(), Box<dyn Error>> {
    let buffer = "Hello World!";
    let mut temp_file = NamedTempFile::new()?;

    temp_file.write_all(b"Test\n")?;

    let mut cmd = Command::cargo_bin("tee")?;
    cmd.arg("-a").arg(temp_file.path()).write_stdin(buffer).output()?;

    let mut file = File::open(temp_file)?;
    let mut file_buffer = String::from("Test\n");
    file.read_to_string(&mut file_buffer)?;

    assert_eq!("Test\nHello World!".to_owned(), file_buffer);

    Ok(())
}
