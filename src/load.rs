use std::io::{Read, Seek, SeekFrom};

use super::*;

impl Table {
  pub fn load(&mut self) -> Result<(), std::io::Error> {
    let mut pos = 0;
    loop {
      // Get row header
      let mut freesize = 0;
      loop {
        let mut header = [0u8; 1];
        match self.file.read_exact(&mut header) { // EOF?
          Ok(()) => (),
          Err(err) => match err.kind() {
            std::io::ErrorKind::UnexpectedEof => {return Ok(());},
            _ => {return Err(err)}
          }
        }
        if header[0] == ROWHEADER {
          break;
        }

        pos += 1;
        freesize += 1;
      }
      if freesize > 0 {
        self.freeChunks.push(FreeChunk { start: pos - freesize, size: freesize });
      }

      // Read column by column
      for col in self.cols.iter() {
        let colpos = pos;
        match col.typ {
          ColType::INT | ColType::FLOAT => {
            if col.index {
              let mut buf = [0u8; 8];
              self.file.read_exact(&mut buf)?;
              let c = self.index.get_mut(&col.name).unwrap();
              let k = match col.typ {
                ColType::INT => Value::INT(i64::from_le_bytes(buf)),
                ColType::FLOAT => Value::FLOAT(Float(f64::from_le_bytes(buf))),
                _ => unreachable!(),
              };
              if c.contains_key(&k) {
                c.get_mut(&k).unwrap().push(colpos);
              } else {
                c.insert(k, vec![colpos]);
              }
            } else {
              self.file.seek(SeekFrom::Current(8))?;
            }

            pos += 8;
          }
          ColType::STRING => {
            let mut size_buf = [0u8; 8];
            self.file.read_exact(&mut size_buf)?;
            let size = u64::from_le_bytes(size_buf) as usize;
            pos += 8 + size;

            if col.index {
              // Read string
              let mut buf = vec![0u8; size];
              self.file.read_exact(buf.as_mut_slice())?;
              let k = Value::STRING(String::from_utf8(buf).unwrap());

              // Insert
              let c = self.index.get_mut(&col.name).unwrap();
              if c.contains_key(&k) {
                c.get_mut(&k).unwrap().push(colpos);
              } else {
                c.insert(k, vec![colpos]);
              }
            } else {
              self.file.seek(SeekFrom::Current(size as i64))?;
            }
          }
        }
      }
    }
  }
}