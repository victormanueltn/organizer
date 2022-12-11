use crate::task::Task;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Data {
    pub(crate) tasks: Vec<Task>,
}
