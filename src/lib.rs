use std::{fs::{File, OpenOptions}, collections::HashMap, hash};
mod load;
mod insert;

const ROWHEADER: u8 = 0b10000000;

mod test;

pub struct Table {
    file: File,
    cols: Vec<Column>,

    index: HashMap<String, HashMap<Value, Vec<usize>>>, // map[col]map[value]locs   
    freeChunks: Vec<FreeChunk>,
}

struct FreeChunk {
    start: usize,
    size: usize,
}

#[derive(PartialEq)]
pub enum ColType {
    INT, // i64
    FLOAT, // f64
    STRING,
}

pub struct Column {
    pub typ: ColType,
    pub name: String,
    pub index: bool, // TODO: Use b-tree for index
}

impl Column {
    pub fn new(name: String, typ: ColType, index: bool) -> Self {
        Self {name, typ, index}
    }
}

// https://stackoverflow.com/a/39647997/11388343
#[derive(Debug, Copy, Clone)]
pub struct Float(f64);
impl Float {
    fn key(&self) -> u64 {
        self.0.to_bits()
    }
}
impl hash::Hash for Float {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.key().hash(state)
    }
}
impl PartialEq for Float {
    fn eq(&self, other: &Float) -> bool {
        self.key() == other.key()
    }
}
impl Eq for Float {}

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Value {
    INT(i64),
    FLOAT(Float),
    STRING(String),
}

impl Value {
    pub fn col_type(&self) -> ColType {
        match self {
            Self::INT(_) => ColType::INT,
            Self::FLOAT(_) => ColType::FLOAT,
            Self::STRING(_) => ColType::STRING,
        }   
    }
}

impl Table {
    pub fn new(path: &str, cols: Vec<Column>) -> Result<Self, std::io::Error> {
        let mut val = Self {
            file: OpenOptions::new().read(true).write(true).open(path)?, 
            cols,
            index: HashMap::new(),
            freeChunks: Vec::new(),
        };
        for col in val.cols.iter() {
            if col.index {
                val.index.insert(col.name.clone(), HashMap::new());
            }
        }
        val.load()?;
        Ok(val)
    }
}