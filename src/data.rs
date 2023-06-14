use crate::task::Task;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct Data {
    pub(crate) tasks: Vec<Task>,
    pub(crate) filters: Filters,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Filters {
    pub complete: bool,
    pub todo: bool,
}

#[derive(Debug)]
pub struct FileError {
    pub message: String,
    pub kind: FileErrorKind,
}

#[derive(Debug)]
pub enum FileErrorKind {
    Load,
    Serialization,
    Write,
}

impl From<serde_json::Error> for FileError {
    fn from(original_error: serde_json::Error) -> Self {
        FileError {
            message: original_error.to_string(),
            kind: FileErrorKind::Serialization,
        }
    }
}

impl Data {
    pub(crate) fn save(&self, file_name: &str) -> Result<(), FileError> {
        let serialized_data = serde_json::to_string(&self).unwrap();
        let file_name = Self::add_extension_if_missing(file_name)?;

        std::fs::write(file_name, serialized_data).map_err(|_| FileError {
            message: "Problem saving file.".to_string(),
            kind: FileErrorKind::Write,
        })
    }
    pub(crate) fn load(file_name: &str) -> Result<Data, FileError> {
        let file_name = Self::add_extension_if_missing(file_name)?;
        let serialized_data = std::fs::read_to_string(file_name).map_err(|_| FileError {
            message: "Problem loading file".to_string(),
            kind: FileErrorKind::Load,
        })?;
        let data = serde_json::from_str::<Self>(&serialized_data)?;
        Ok(data)
    }

    fn add_extension_if_missing(file_name: &str) -> Result<String, FileError> {
        let extension = std::path::Path::new(file_name).extension();
        let mut file_name = file_name.to_string();

        match extension {
            Some(extension) => {
                if extension != "ogz" {
                    return Err(FileError {
                        message: "Wrong file extension. Expected '.ogz'".to_string(),
                        kind: FileErrorKind::Write,
                    });
                }
            }
            None => file_name.push_str(".ogz"),
        }

        if file_name.contains('.') {
        } else {
            file_name.push_str(".ogz")
        }
        Ok(file_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_load_returns_same_data() {
        let data = Data {
            tasks: vec![Task::new(0_usize), Task::new(1_usize)],
            filters: Filters {
                todo: true,
                complete: true,
            },
        };

        let file_name = "test_data.ogz";

        data.save(file_name).unwrap();
        let loaded_data = Data::load(file_name).unwrap();

        assert_eq!(data, loaded_data);
    }

    #[test]
    fn load_inexistent_file() {
        let loaded_data = Data::load("inexsistent.ogz");

        assert!(matches!(loaded_data.unwrap_err().kind, FileErrorKind::Load));
    }

    #[test]
    fn load_invalid_file() {
        let file_name = "test_invalid_data.ogz";
        std::fs::write(file_name, "{\"tasks\":[{{\"id\":0,\"task_completed\":false,\"description\":\"\",\"state\":\"Idle\"}]}").unwrap();

        let loaded_data = Data::load(file_name);

        assert!(matches!(
            loaded_data.unwrap_err().kind,
            FileErrorKind::Serialization
        ));
    }

    #[test]
    fn save_to_inexistent_folder() {
        let data = Data {
            tasks: vec![Task::new(0_usize), Task::new(1_usize)],
            filters: Filters {
                todo: true,
                complete: true,
            },
        };

        let save_result = data.save("./inexistent_directory/data.ogz");

        assert!(matches!(
            save_result.unwrap_err().kind,
            FileErrorKind::Write
        ));
    }
}
