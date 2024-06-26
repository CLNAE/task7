use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};

fn main() -> io::Result<()> {
    // specify the input file path
    let input_file_path = "D:/PycharmProjects/task6/mat.in.x"; // this is the path where my
    // file is located. it should be updated with the correct input file path for any other user

    // determines the output file path based on the input file extension
    let output_file_path = if input_file_path.ends_with(".in.x") {
        input_file_path.trim_end_matches(".in.x").to_string() + ".in"
    } else if input_file_path.ends_with(".in") {
        input_file_path.to_string() + ".x"
    } else {
        eprintln!("Invalid file extension. This script only works with files that have the .in or \
        .in.x extension");
        return Ok(());
    };

    // opens the input file for reading
    let file_input = File::open(input_file_path)?;
    let reader = io::BufReader::new(file_input);

    // creates the output file for writing
    let file_output = File::create(output_file_path)?;
    let mut writer = BufWriter::new(file_output);
    //mut=mutable; it allows its value to be changed later in the code

    // processes each line in the input file
    for line in reader.lines() {
        let line = line?; // handles any potential errors reading the line
        let mut parts = line.split(':'); // splits the line on ":"
        if let Some(before_colon) = parts.next() {
            if let Some(after_colon) = parts.next() {
                let conversion_result = if input_file_path.ends_with(".in.x") {
                    hexadecimal_to_binary(after_colon.trim())
                } else {
                    binary_to_hexadecimal(after_colon.trim())
                };

                // writes the result to the output file
                if let Some(converted) = conversion_result {
                    writeln!(&mut writer, "{}:{}", before_colon, converted)?;
                } else {
                    writeln!(&mut writer, "{}: conversion error", before_colon)?; // writes
                    // 'conversion error' if conversion fails
                }
            }
        }
    }

    Ok(())
}

fn binary_to_hexadecimal(binary: &str) -> Option<String> {
    //&str = this function receives a reference to a string
    //-> Option<String> = this function returns a string
    // checks if the input string is empty or contains any characters that are not valid binary
    // digits (0 or 1).
    if binary.is_empty() || binary.chars().any(|c| !c.is_digit(2)) {
        return None; // if invalid, return None to indicate failure.
    }

    // let mut binary_string = binary.to_string(); // converts the input string to a mutable
    // // string.
    // while binary_string.len() % 4 != 0 {
    //     binary_string.insert(0, '0');
    //     //if the string's size is not a multiple of 4, it adds "0"s at the beginning
    // }

    let mut hexa = String::new(); //String::new() = creates an empty string
    for chunk in binary.chars().collect::<Vec<_>>().chunks(4) {
        //checks every chunk of 4 digits
        //chars() iterates over the characters in the string slice; collect() collects the
        // characters from the iterator; Vec specifies the type of collection
        //chunks(4) = a chunk has 4 charactes/digits
        let binary_slice: String = chunk.iter().collect(); //a chunk of 4 digits is allocated to
        // bin_str
        let num = u32::from_str_radix(&binary_slice, 2).unwrap(); //u32 = 32bit unsigned
        // integer; from_str_radix converts a string slice into an integer in a given base
        // (in this case 2)
        //if it is "Some" unwrap returns the result. if it is "None", it stops the script
        let hexa_digit = format!("{:X}",num); //format! formats "num" into hexadecimal string
        hexa.push_str(&hexa_digit); //adds "hexa_digit"(substring) at the end of "hexa"(main string)
    }

    Some(hexa) //the function returns something (a value) and places it into the "hexa" variable
}

fn hexadecimal_to_binary(hexa: &str) -> Option<String> {
    //&str = this function receives a reference to a string
    //-> Option<String> = this function returns a string
    //checks iof the input string is empty or contains any cahracters that are not valid hexadecimal
    // digits (0-9, A-F, a-f).
    if hexa.is_empty() || hexa.chars().any(|c| !c.is_digit(16)) {
        return None; // If invalid, return None to indicate failure.
    }

    let mut binary = String::new(); // initializes an empty string to hold the resulting
    // binary string.

    for hex_char in hexa.chars() {
        // iterates over each character in the hexadecimal string.
        let num = u32::from_str_radix(&hex_char.to_string(), 16).unwrap(); //converts the
        // hexadecimal character to a u32 integer.
        let binary_str = format!("{:04b}", num); // formats the integer as a 4-bit binary
        // string.
        binary.push_str(&binary_str); // appends the binary string to the result.
    }

    Some(binary) //the function returns something (a value) and places it into the "binary" variable
}
