pub enum Author {
  User,
  Assistent
}

pub struct Message {
  pub author: Author,
  pub content: String
}

pub struct Chat {
  pub messages: Vec<Message>,
}

/// Represents chat with Bot and User only
/// Check ChatCompletion for unlimited users count
pub trait BotChatCompletion {
  fn complete(chat: Chat) -> String;
}