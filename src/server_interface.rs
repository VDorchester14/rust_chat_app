use crate::user_state::UserState;
use crate::message::Message;

pub struct ServerInterface{
    pub message_history: Vec<Message>,
    pub user_list: Vec<UserState>
}

impl ServerInterface {
    pub fn send_message(&mut self, message: Message) {
        self.message_history.push(message);
    }

    pub fn poll_message(&mut self){

    }
}

impl Default for ServerInterface {
    fn default() -> Self {
        Self {
            message_history: Vec::new(),
            user_list: Vec::new(),
        }
    }
}