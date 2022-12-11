use crate::task::Task;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct Data {
    pub(crate) tasks: Vec<Task>,
}

impl Data {
    fn _save(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        let serialized_data = serde_json::to_string(&self)?;
        std::fs::write(file_name, serialized_data)?;
        Ok(())
    }
    fn _load(file_name: &str) -> Result<Data, Box<dyn Error>> {
        let serialized_data = std::fs::read_to_string(file_name)?;
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
            tasks: vec![Task::new(0_usize)],
        };

        data._save("data.json");
        let loaded_data = Data::_load("data.json").unwrap();

        assert_eq!(data, loaded_data);
    }
}
