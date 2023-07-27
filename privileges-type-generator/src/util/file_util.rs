use std::io::Write;

pub fn write_line(file: &mut std::fs::File, indents: usize, content: String) {
    let indent = "\t";
    let _ = file.write(indent.repeat(indents).as_bytes());
    let _ = file.write(content.as_bytes());
    let _ = file.write(b",\n");
}