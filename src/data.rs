use crate::task::Task;
use crate::task::{self};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct Data {
    pub(crate) tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddTask,
    TaskMessage(usize, task::Message),
}

#[derive(Debug)]
pub(crate) struct FileError {
    message: String,
    kind: FileErrorKind,
}

#[derive(Debug)]
enum FileErrorKind {
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
    fn save(&self, file_name: &str) -> Result<(), FileError> {
        let serialized_data = serde_json::to_string(&self).unwrap();

        std::fs::write(file_name, serialized_data).map_err(|_| FileError {
            message: "Problem saving file.".to_string(),
            kind: FileErrorKind::Write,
        })
    }
    fn load(file_name: &str) -> Result<Data, FileError> {
        let serialized_data = std::fs::read_to_string(file_name).map_err(|_| FileError {
            message: "Problem loading file".to_string(),
            kind: FileErrorKind::Load,
        })?;
        let data = serde_json::from_str::<Self>(&serialized_data)?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_load_returns_same_data() {
        let data = Data {
            tasks: vec![Task::new(0_usize), Task::new(1_usize)],
        };

        let file_name = "test_data.json";

        data.save(file_name).unwrap();
        let loaded_data = Data::load(file_name).unwrap();

        assert_eq!(data, loaded_data);
    }

    #[test]
    fn load_inexistent_file() {
        let loaded_data = Data::load("inexsistent.json");

        assert!(matches!(loaded_data.unwrap_err().kind, FileErrorKind::Load));
    }

    #[test]
    fn load_invalid_file() {
        let file_name = "test_invalid_data.json";
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
        };

        let save_result = data.save("./inexistent_directory/data.json");

        assert!(matches!(
            save_result.unwrap_err().kind,
            FileErrorKind::Write
        ));
    }
}
