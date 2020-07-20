use std::{fs, env};
use minidom::Element;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut a = env::current_dir().expect("Something went wrong");
    println!("{}", a.as_os_str().to_string_lossy());
    a.push("ucd.all.flat.xml");

    println!("{}", a.as_os_str().to_string_lossy());

    let contents = fs::read_to_string(a)
        .expect("File read error");

    let root: Element = contents.parse().unwrap();
    let mut data: [u8; 139264] = [0; 139264];

    for child in root.children() {
        if child.name().eq("repertoire") {
            for subchild in child.children() {
                if let Some(cp) = subchild.attr("cp") {
                    if subchild.attr("Upper").unwrap() == "Y" {
                        let z =
                            i64::from_str_radix(cp, 16)
                                .expect("An error occured while decoding hex number");

                        let index = z >> 3;
                        let ae = z - (index << 3);

                        data[index as usize] += 1 << ae;
                    }
                }
            }
        }
    }

    let mut pos = 0;
    let mut buffer = File::create("out.bin").expect("Failed to create file");

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])
            .expect("Failed to write data");
        pos += bytes_written;
    }

    buffer.flush().expect("Failed to flush the file down the toilet");
}
