use std::io::{BufReader, BufWriter, Error, ErrorKind, Read, Result, Write};

type BufData = u64;
const BYTE_WIDTH: u8 = 8;

pub fn read_one_byte<T: Read>(stream: &mut T) -> Result<u8> {
    let mut value = [0; 1];
    if stream.read(&mut value)? > 0 {
        Ok(value[0])
    } else {
        Err(Error::new(
            ErrorKind::UnexpectedEof,
            "Reached the end of the file",
        ))
    }
}

struct BitBuf {
    data: BufData,
    offset: u8,
}

impl BitBuf {
    fn new() -> Self {
        BitBuf { data: 0, offset: 0 }
    }
}

pub struct BitBufWriter<T: Write> {
    buf: BitBuf,
    writer: BufWriter<T>,
}

impl<T: Write> BitBufWriter<T> {
    pub fn new(stream: T) -> Self {
        BitBufWriter {
            buf: BitBuf::new(),
            writer: BufWriter::new(stream),
        }
    }

    pub fn write(&mut self, data: BufData, width: u8) -> Result<()> {
        self.buf.data += (data & ((1 << width) - 1)) << self.buf.offset;
        self.buf.offset += width;

        while self.buf.offset >= BYTE_WIDTH {
            let value = self.buf.data & ((1 << BYTE_WIDTH) - 1);
            self.writer.write(&[value as u8])?;
            self.buf.offset -= BYTE_WIDTH;
            self.buf.data >>= BYTE_WIDTH;
        }

        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.writer.write(&[self.buf.data as u8])?;
        self.writer.flush()?;
        Ok(())
    }
}

pub struct BitBufReader<T: Read> {
    buf: BitBuf,
    reader: BufReader<T>,
}

impl<T: Read> BitBufReader<T> {
    pub fn new(stream: T) -> Self {
        BitBufReader {
            buf: BitBuf::new(),
            reader: BufReader::new(stream),
        }
    }

    pub fn read(&mut self, width: u8) -> Result<BufData> {
        while self.buf.offset < width {
            let value = read_one_byte(&mut self.reader)?;
            self.buf.data += (value as u64) << self.buf.offset;
            self.buf.offset += BYTE_WIDTH;
        }

        let result = self.buf.data & ((1 << width) - 1);
        self.buf.data >>= width;
        self.buf.offset -= width;

        Ok(result)
    }
}
