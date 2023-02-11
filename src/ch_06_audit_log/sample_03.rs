use chrono::{DateTime, Utc};
use std::io::Write;
use std::{
    fs::{read_to_string, File},
    path::{Path, PathBuf},
};
struct AuditManager {
    max_entries_perfile: usize,
}

struct FileContent {
    lines: Vec<String>,
    file_name: String,
}

struct FileUpdate {
    path: String,
    content: String,
}

impl AuditManager {
    pub fn add_record(
        &self,
        files: Vec<FileContent>,
        visitor_name: &str,
        time_of_visit: &DateTime<Utc>,
    ) -> FileUpdate {
        // TODO: sort
        let mut sorted = files;

        let new_record = format!("{visitor_name}; {time_of_visit}");

        if sorted.len() == 0 {
            return FileUpdate {
                path: "audit_1.txt".to_owned(),
                content: new_record,
            };
        }

        let current_file = sorted.last_mut().unwrap();

        if current_file.lines.len() < self.max_entries_perfile {
            current_file.lines.push(new_record);
            let new_content = current_file.lines.join("\n");
            let file_name = current_file.file_name.clone();
            return FileUpdate {
                path: file_name,
                content: new_content,
            };
        } else {
            let new_index = current_file.lines.len();
            let new_name = format!("audit_{new_index}.txt");
            return FileUpdate {
                path: new_name,
                content: new_record,
            };
        }
    }
}

struct Persister {}

impl Persister {
    pub fn read_directory(&self, directory_name: &str) -> Vec<FileContent> {
        Path::new(directory_name)
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
            .map(|file_name| {
                let content = read_to_string(&file_name)
                    .expect(&format!("failed to read file: {}", file_name));

                FileContent {
                    file_name,
                    lines: content.lines().map(|l| l.to_string()).collect::<Vec<_>>(),
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn apply_update(&self, directory_name: &str, update: FileUpdate) {
        let file_path: PathBuf = [directory_name, &update.path].iter().collect();
        let mut file =
            File::open(&file_path).expect(&format!("failed to open file: {:?}", file_path));
        file.write_all(update.content.as_bytes())
            .expect(&format!("failed to write file: {:?}", file_path));
    }
}

struct ApplicationService {
    directory_name: String,
    audit_manager: AuditManager,
    persister: Persister,
}

impl ApplicationService {
    fn add_record(&self, visitor_name: &str, time_of_visit: &DateTime<Utc>) {
        let files = self.persister.read_directory(&self.directory_name);
        let update = self
            .audit_manager
            .add_record(files, visitor_name, time_of_visit);
        self.persister.apply_update(&self.directory_name, update);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_file_is_created_when_the_current_file_overflows() {
        let sut = AuditManager {
            max_entries_perfile: 3,
        };
        let files = vec![
            FileContent {
                file_name: "audit_1.txt".to_owned(),
                lines: vec![],
            },
            FileContent {
                file_name: "audit_2.txt".to_owned(),
                lines: vec![
                    "Peter; 2019-04-06T16:30:00".to_owned(),
                    "Jane; 2019-04-06T16:40:00".to_owned(),
                    "Jack; 2019-04-06T17:00:00".to_owned(),
                ],
            },
        ];
        let update = sut.add_record(
            files,
            "Alice",
            &"2014-11-28T12:00:09Z".parse::<DateTime<Utc>>().unwrap(),
        );

        assert_eq!("audit_3.txt", update.path);
        assert_eq!("Alice; 2014-11-28 12:00:09 UTC", update.content);
    }
}
