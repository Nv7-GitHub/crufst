use std::io::{Seek, SeekFrom, Write};

use super::*;

#[derive(Debug)]
pub enum InsertError {
  IOError(std::io::Error),
  InvalidType(usize),
  InvalidValueCount,
}

impl From<std::io::Error> for InsertError {
    fn from(v: std::io::Error) -> Self {
      Self::IOError(v)
    }
}

impl Table {
  pub fn insert(&mut self, vals: Vec<Value>) -> Result<(), InsertError> {
    // Type-check
    if vals.len() != self.cols.len() {
      return Err(InsertError::InvalidValueCount);
    }
    for (i, col) in self.cols.iter().enumerate() {
      if col.typ != vals[i].col_type() {
        return Err(InsertError::InvalidType(i))
      }
    }

    // Calculate bytes
    let mut bytes = vec![ROWHEADER];
    for val in vals {
      match val {
        Value::INT(v) => bytes.extend_from_slice(&v.to_le_bytes()),
        Value::FLOAT(v) => bytes.extend_from_slice(&v.0.to_le_bytes()),
        Value::STRING(v) => {
          bytes.extend_from_slice(&(v.len() as u64).to_le_bytes());
          bytes.extend_from_slice(v.as_bytes());
        }
      }
    }

    // Search for space
    for space in self.freeChunks.iter_mut() {
      if space.size >= bytes.len() {
        self.file.seek(SeekFrom::Start(space.start as u64))?;
        space.size -= bytes.len();
        space.start += bytes.len();
        self.file.write_all(bytes.as_slice())?;
        return Ok(());
      }
    }

    // Append to end
    self.file.seek(SeekFrom::End(0))?;
    self.file.write(bytes.as_slice())?;
    Ok(())
  }
}