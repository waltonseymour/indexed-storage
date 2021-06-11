use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

/// Database for efficiently storing and retrieving arbitrary length byte entries
pub struct DB<T: Write + Read + Seek> {
    data: T,
    index: T,
    /// Current offset of data file. Only used during writing
    offset: u64,
}

pub fn new_reader(path: &str) -> DB<File> {
    let path = std::path::Path::new(path);

    let index_file = std::fs::File::open(path.join("index.bin")).unwrap();
    let data_file = std::fs::File::open(path.join("data.bin")).unwrap();

    DB {
        index: index_file,
        data: data_file,
        offset: 0,
    }
}

pub fn new_writer(path: &str) -> DB<File> {
    let path = std::path::Path::new(path);
    std::fs::create_dir(path);

    let index_file = std::fs::File::create(path.join("index.bin")).unwrap();
    let data_file = std::fs::File::create(path.join("data.bin")).unwrap();

    DB {
        index: index_file,
        data: data_file,
        offset: 0,
    }
}

impl<T: Write + Read + Seek> DB<T> {
    pub fn new(data: T, index: T) -> DB<T> {
        DB {
            index: index,
            data: data,
            offset: 0,
        }
    }

    pub fn write_all(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.data.write_all(data)?;

        let length = data.len() as u64;

        // write offset
        self.index.write_all(&self.offset.to_le_bytes())?;
        // write length
        self.index.write_all(&length.to_le_bytes())?;

        // increment offset
        self.offset += length;

        Ok(())
    }

    pub fn read(&mut self, index: u64) -> std::io::Result<Vec<u8>> {
        self.index.seek(std::io::SeekFrom::Start(index * 16))?;

        let mut buf = [0; 16];
        self.index.read_exact(&mut buf)?;

        let data: [u64; 2];
        unsafe {
            data = std::mem::transmute::<[u8; 16], [u64; 2]>(buf);
        }

        let offset = data[0];
        let length = data[1];

        let mut buf = vec![0; length as usize];
        self.data.seek(std::io::SeekFrom::Start(offset))?;
        self.data.read_exact(&mut buf)?;

        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_and_read() {
        let dir = std::env::temp_dir();
        let path = dir.as_path().to_str().unwrap();
        let mut writer = new_writer(path);

        writer.write_all(&[1, 2, 3]).expect("could not write data");
        writer.write_all(&[4, 5, 6]).expect("could not write data");

        let mut reader = new_reader(path);

        let data = reader.read(0).expect("could not read");
        assert_eq!(data, [1, 2, 3]);
        let data = reader.read(1).expect("could not read");
        assert_eq!(data, [4, 5, 6]);
    }
}
