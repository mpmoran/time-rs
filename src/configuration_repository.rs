use ini::Ini;
use std::fs::File;
use std::io::{BufReader, BufWriter, Seek, Write};

const SECTION: &str = "default";

pub struct ConfigurationRepository {
    file: File,
    store: Ini,
}

impl ConfigurationRepository {
    pub fn new(file: File) -> ConfigurationRepository {
        let mut repo = ConfigurationRepository {
            file,
            store: Ini::new(),
        };
        repo.read();

        repo
    }

    pub fn set(&mut self, name: &str, value: String) {
        self.store
            .with_section(Some(SECTION))
            .set(String::from(name), value);
    }

    // fn close(&self) {
    //     unimplemented!()
    // }

    fn read(&mut self) {
        self.file.rewind().unwrap();
        let mut reader = BufReader::new(&self.file);
        self.store = Ini::read_from(&mut reader).unwrap();
    }

    pub fn get(&mut self, name: &str) -> String {
        let value = self.store.get_from(Some(SECTION), name).unwrap();
        String::from(value)
    }

    pub fn write(&mut self) {
        self.file.rewind().unwrap();
        let mut writer = BufWriter::new(&self.file);
        self.store.write_to(&mut writer).unwrap();
        writer.flush().unwrap();
    }
}
