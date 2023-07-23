use std::collections::HashMap;

pub type Code = u32;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct DictEntry(pub Option<Code>, pub u8);
const FIRST_CODE: Code = 256;
const START_WIDTH: u8 = 9;

pub struct Dict {
    pub table: Vec<DictEntry>,
    inversed_table: HashMap<DictEntry, Code>,
    pub width: u8,
    read: bool,
}

impl Dict {
    pub fn new(read: bool) -> Self {
        let mut data = Vec::new();

        for i in 0..FIRST_CODE {
            data.push(DictEntry(None, i as u8))
        }

        Dict {
            table: data,
            inversed_table: HashMap::new(),
            width: START_WIDTH,
            read,
        }
    }

    pub fn find(&self, code: Code, byte: u8) -> Option<Code> {
        let entry = DictEntry(Some(code), byte);
        match self.inversed_table.get(&entry) {
            None => None,
            Some(result) => {
                let result_entry = self.table.get(*result as usize).unwrap();

                if result_entry.0.is_some()
                    && result_entry.0.unwrap() == code
                    && result_entry.1 == byte
                {
                    Some(*result)
                } else {
                    None
                }
            }
        }
    }

    pub fn insert(&mut self, code: Code, byte: u8) {
        let entry = DictEntry(Some(code), byte);
        self.table.push(entry);
        self.inversed_table
            .insert(entry, self.table.len() as Code - 1);

        if (self.read && self.table.len() == (1 << self.width) - 2)
            || self.table.len() == (1 << self.width)
        {
            self.width += 1;
        }
    }
}
