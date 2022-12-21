use crate::data::{self, Data};

impl ToIced for Data {
    type Message = Data::Message;
    fn view(&self) -> Element<Data::Message> {}
}
