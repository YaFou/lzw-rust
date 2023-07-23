use hashbrown::HashMap;

pub type Code = u32;
pub type Byte = u8;
type Width = u8;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct DictEntry(pub Option<Code>, pub Byte);
const FIRST_CODE: Code = 256;
const START_WIDTH: Width = 9;

#[derive(PartialEq)]
pub enum State {
    WRITE,
    READ,
}

pub struct Dict {
    pub table: Vec<DictEntry>,
    inversed_table: HashMap<DictEntry, Code>,
    pub width: Width,
    state: State,
}

impl Dict {
    pub fn new(state: State) -> Self {
        let mut data = Vec::new();

        for i in 0..FIRST_CODE {
            data.push(DictEntry(None, i.try_into().unwrap()));
        }

        Dict {
            table: data,
            inversed_table: HashMap::new(),
            width: START_WIDTH,
            state,
        }
    }

    pub fn find(&self, code: Code, byte: Byte) -> Option<Code> {
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

    pub fn insert(&mut self, code: Code, byte: Byte) {
        let entry = DictEntry(Some(code), byte);
        self.table.push(entry);

        if self.state == State::WRITE {
            self.inversed_table
                .insert(entry, (self.table.len() - 1).try_into().unwrap());
        }

        if (self.state == State::READ && self.table.len() == (1 << self.width) - 2)
            || self.table.len() == (1 << self.width)
        {
            self.width += 1;
        }
    }
}
