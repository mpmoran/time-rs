use crate::csv_repository::CsvRepository;
use crate::stopwatch::Stopwatch;
use crate::task_record::TaskRecord;
use std::fs::File;

pub struct TimeRs {
    time_keeper: Stopwatch,
    output: CsvRepository,
}

impl TimeRs {
    pub fn new(output_file: File) -> TimeRs {
        let output_repo = CsvRepository::new(output_file);
        TimeRs {
            time_keeper: Stopwatch::new(),
            output: output_repo,
        }
    }

    pub fn start(&mut self) {
        self.time_keeper.start();
    }

    pub fn advance(&mut self, amount: u64) {
        self.time_keeper.advance(amount);
    }

    pub fn get_length_for_human(&self) -> String {
        let length = self.time_keeper.get_length_as_uint();
        let minutes = length / 60;
        let seconds = length % 60;

        format!("{:02}:{:02}", minutes, seconds)
    }

    pub fn get_length_for_record(&self) -> String {
        let length = self.time_keeper.get_length_as_f64();

        format!("{:.2}", length)
    }

    pub fn reset(&mut self) {
        self.time_keeper.reset();
    }

    pub fn stop(&mut self) {
        self.time_keeper.stop();
    }

    pub fn write(&mut self, task: String) {
        let length_string = self.get_length_for_record();
        let new_task_record = TaskRecord::for_today(length_string, task);
        let mut task_records: Vec<TaskRecord> = self.output.read();
        task_records.push(new_task_record);
        self.output.write(task_records);
    }
}
