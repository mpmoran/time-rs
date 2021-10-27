use crate::task_record::TaskRecord;
use csv::{Reader, Writer};
use std::fs::File;
use std::io::Seek;

pub struct CsvRepository {
    file: File,
}

impl CsvRepository {
    pub fn new(file: File) -> CsvRepository {
        CsvRepository { file }
    }

    // TODO make this return a Vec of any type
    pub fn read(&mut self) -> Vec<TaskRecord> {
        self.file.rewind().unwrap();
        let mut records: Vec<TaskRecord> = Vec::new();
        let mut reader = Reader::from_reader(&self.file);
        for result in reader.deserialize() {
            let record: TaskRecord = result.unwrap();
            records.push(record);
        }

        records
    }

    // TODO make this write a Vec of any type
    pub fn write(&mut self, records: Vec<TaskRecord>) {
        self.file.rewind().unwrap();
        let mut writer = Writer::from_writer(&self.file);
        for record in records {
            writer.serialize(record).unwrap();
        }
        writer.flush().unwrap();
    }
}
