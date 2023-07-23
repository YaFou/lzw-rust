use std::{io::{Read, Result, Write}, thread, time::Duration};

use crate::{
    bit_buffer::{read_one_byte, BitBufReader, BitBufWriter},
    dict::{Code, Dict},
};

pub fn compress<T: Read, U: Write>(mut in_stream: T, out_stream: U) -> Result<()> {
    let mut writer = BitBufWriter::new(out_stream);
    let mut dict = Dict::new(false);
    let mut last_code = Some(read_one_byte(&mut in_stream)? as Code);
    let mut result = read_one_byte(&mut in_stream);

    while let Ok(c) = result {
        match last_code {
            None => {
                last_code = Some(c as Code);
                result = read_one_byte(&mut in_stream);
            }
            Some(code) => match dict.find(code, c) {
                None => {
                    dict.insert(code, c);
                    writer.write(code.into(), dict.width)?;
                    last_code = None;
                }
                Some(inner_code) => {
                    last_code = Some(inner_code);
                    result = read_one_byte(&mut in_stream);
                }
            },
        }
    }

    writer.write(last_code.unwrap().into(), dict.width)?;
    writer.flush()?;

    Ok(())
}

fn decode<T: Write>(dict: &Dict, stream: &mut T, code: Code) -> Result<u8> {
    let mut pattern = Vec::new();
    // let i: usize = 0;
    let mut entry = dict.table.get(code as usize).unwrap();

    while let Some(pointer) = entry.0 {
        pattern.push(entry.1);
        entry = dict.table.get(pointer as usize).unwrap();
    }

    pattern.push(entry.1);
    pattern.reverse();
    for c in pattern.iter() {
        stream.write(&[*c])?;
    }
    Ok(*pattern.get(0).unwrap())
}

pub fn decompress<T: Read, U: Write>(in_stream: T, mut out_stream: U) -> Result<()> {
    let mut reader = BitBufReader::new(in_stream);
    let mut dict = Dict::new(true);
    let mut result = reader.read(dict.width);

    if result.is_err() {
        return Ok(());
    }

    let current_code = result.unwrap() as Code;
    let mut c = decode(&dict, &mut out_stream, current_code)?;
    let mut last_code = current_code;
    result = reader.read(dict.width);

    while let Ok(code) = result {
        if (code as usize) < dict.table.len() {
            c = decode(&dict, &mut out_stream, code as Code)?;
        } else if code as usize == dict.table.len() {
            c = decode(&dict, &mut out_stream, last_code)?;
            out_stream.write(&[c])?;
        }
        
        dict.insert(last_code, c);
        last_code = code as Code;
        result = reader.read(dict.width);
    }

    Ok(())
}
