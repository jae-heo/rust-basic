use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

pub fn test_panic(){    
    let test_file_result = File::open("test.txt");

    let test_file = match test_file_result {
        Ok(file) => {
            println!("file found!");
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(fc) => {
                    println!("file created because there was no file");
                    fc
                },
                Err(e) => panic!("Error while creating file!")
            },
            fdzz => panic!("Unexpected error!"),
        }
    };
}

pub fn test_result() -> Result<String, io::Error> {
    let mut file: File = File::open("test.txt")?;
    let mut test_string = String::new();
    file.read_to_string(&mut test_string)?;
    Ok(test_string)
}