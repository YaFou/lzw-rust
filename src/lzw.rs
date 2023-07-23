use std::io::{BufWriter, Read, Result, Write, BufReader};

use crate::{
    bit_buffer::{read_one_byte, BitBufReader, BitBufWriter},
    dict::{Byte, Code, Dict, State},
};

pub fn compress<T: Read, U: Write>(in_stream: T, out_stream: U) -> Result<()> {
    let mut in_buf = BufReader::new(in_stream);
    let mut writer = BitBufWriter::new(out_stream);
    let mut dict = Dict::new(State::WRITE);
    let mut last_code = Some(read_one_byte(&mut in_buf)?.into());
    let mut result = read_one_byte(&mut in_buf);

    while let Ok(c) = result {
        match last_code {
            None => {
                last_code = Some(c.into());
                result = read_one_byte(&mut in_buf);
            }
            Some(code) => match dict.find(code, c) {
                None => {
                    dict.insert(code, c);
                    writer.write(code.into(), dict.width)?;
                    last_code = None;
                }
                Some(inner_code) => {
                    last_code = Some(inner_code);
                    result = read_one_byte(&mut in_buf);
                }
            },
        }
    }

    writer.write(last_code.unwrap().into(), dict.width)?;
    writer.flush()?;

    Ok(())
}

fn decode<T: Write>(dict: &Dict, stream: &mut T, code: Code) -> Result<Byte> {
    let mut pattern = Vec::new();
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

pub fn decompress<T: Read, U: Write>(in_stream: T, out_stream: U) -> Result<()> {
    let mut reader = BitBufReader::new(in_stream);
    let mut dict = Dict::new(State::READ);
    let mut result = reader.read(dict.width);
    let mut out_buf = BufWriter::new(out_stream);

    if result.is_err() {
        return Ok(());
    }

    let current_code = result.unwrap().try_into().unwrap();
    let mut c = decode(&dict, &mut out_buf, current_code)?;
    let mut last_code = current_code;
    result = reader.read(dict.width);

    while let Ok(code) = result {
        if (code as usize) < dict.table.len() {
            c = decode(&dict, &mut out_buf, code.try_into().unwrap())?;
        } else if code as usize == dict.table.len() {
            c = decode(&dict, &mut out_buf, last_code)?;
            out_buf.write(&[c])?;
        }

        dict.insert(last_code, c);
        last_code = code.try_into().unwrap();
        result = reader.read(dict.width);
    }

    Ok(())
}
