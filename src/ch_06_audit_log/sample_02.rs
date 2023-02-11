use chrono::{DateTime, Utc};
use std::path::Path;
use std::path::PathBuf;

trait FileSysmem {
    fn get_files(&self, path: &str) -> Vec<String>;
    fn write_all<P: AsRef<Path>>(&self, path: P, buf: &[u8]);
    fn read_to_string<P: AsRef<Path>>(&self, path: P) -> String;
}

struct AuditManager<F: FileSysmem> {
    max_entries_perfile: usize,
    directory_name: String,
    file_system: F,
}

impl<F: FileSysmem> AuditManager<F> {
    fn add_record(&self, visitor_name: &str, time_of_visit: &DateTime<Utc>) {
        let file_paths = self.file_system.get_files(&self.directory_name);

        // TODO: sort
        let sorted = file_paths;
        let new_record = format!("{}: {:?}", visitor_name, time_of_visit);

        if sorted.len() == 0 {
            let new_file: PathBuf = [self.directory_name.clone(), "audit_1.txt".to_owned()]
                .iter()
                .collect();
            self.file_system.write_all(&new_file, new_record.as_bytes());
            return;
        }

        let current_file_path = sorted.last().unwrap();
        let content = self.file_system.read_to_string(current_file_path);
        let mut lines = content.split("\n").collect::<Vec<_>>();
        if lines.len() < self.max_entries_perfile {
            lines.push(&new_record);
            let new_content = lines.join("\n");
            self.file_system
                .write_all(current_file_path, new_content.as_bytes());
        } else {
            let new_index = sorted.len() + 1;
            let new_name = format!("audit_{new_index}.txt");
            let new_file: PathBuf = [self.directory_name.clone(), new_name.clone()]
                .iter()
                .collect();
            self.file_system.write_all(new_file, new_record.as_bytes());
        }
    }
}
