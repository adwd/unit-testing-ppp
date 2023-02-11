use std::{
    fs::{read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};

struct AuditManager {
    max_entries_perfile: usize,
    directory_name: String,
}

impl AuditManager {
    fn add_record(&self, visitor_name: &str, time_of_visit: &DateTime<Utc>) {
        let path = Path::new(&self.directory_name);
        let file_paths = path
            .read_dir()
            .expect("read_dir call failed")
            .into_iter()
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    entry.file_name().to_str().map(|s| s.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // TODO: sort
        let sorted = file_paths;
        let new_record = format!("{}: {:?}", visitor_name, time_of_visit);

        if sorted.len() == 0 {
            let new_file: PathBuf = [self.directory_name.clone(), "audit_1.txt".to_owned()]
                .iter()
                .collect();
            let mut file =
                File::create(&new_file).expect(&format!("failed to create file: {:?}", new_file));
            file.write_all(new_record.as_bytes())
                .expect(&format!("failed to write file: {:?}", new_file));
            return;
        }

        let current_file_path = sorted.last().unwrap();
        let content = read_to_string(current_file_path)
            .expect(&format!("failed to read file: {}", current_file_path));
        let mut lines = content.split("\n").collect::<Vec<_>>();
        if lines.len() < self.max_entries_perfile {
            lines.push(&new_record);
            let new_content = lines.join("\n");
            let mut file = File::open(current_file_path)
                .expect(&format!("failed to open file: {}", current_file_path));
            file.write_all(new_content.as_bytes())
                .expect(&format!("failed to write file: {}", current_file_path));
        } else {
            let new_index = sorted.len() + 1;
            let new_name = format!("audit_{new_index}.txt");
            let new_file: PathBuf = [self.directory_name.clone(), new_name.clone()]
                .iter()
                .collect();
            let mut file =
                File::create(new_file).expect(&format!("failed to create file :{:?}", new_name));
            file.write_all(new_record.as_bytes())
                .expect(&format!("failed to write file: {}", current_file_path));
        }
    }
}
