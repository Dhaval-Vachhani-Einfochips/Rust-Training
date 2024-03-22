use std::fs::File;
use std::io::Write;

struct FileWriter 
{
    file: File,
}

impl FileWriter 
{
    fn new(filename: &str) -> FileWriter 
    {
        let file = File::create(filename).expect("create failed");
        FileWriter { file }
    }

    fn write(&mut self, data: &[u8]) 
    {
        self.file.write_all(data).expect("write failed");
    }
}

impl Drop for FileWriter 
{
    fn drop(&mut self) 
    {
        self.file.flush().expect("flush failed");
        println!("Closing file!");
    }
}

fn main() 
{
    let mut writer = FileWriter::new("data.txt");
    writer.write(b"Hello, world!\n");
}