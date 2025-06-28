use std::collections::HashMap;
use prompted::input;
use llm4free::service_api::bot_chat_completion::{Author, BotChatCompletion, Chat, Message};

fn main() {
  println!("llm4free chat. Please, read README.MD and LEGAL_NOTICE.MD before using!\n");

  let completors: HashMap<String, Box<dyn BotChatCompletion>> = HashMap::new();


  match completors.get(&input!("Model name: ")) {
    Option::None => println!("Wrong model type!"),
    Option::Some(completor) => {
      let mut chat = Chat {
        messages: vec![]
      };

      println!("Chat started!");

      loop {
        let message = input!("> ");
        chat.messages.push(Message {
          author: Author::User,
          content: message
        });

        let answer = completor.complete(&chat);
        println!("< {}", answer);
        chat.messages.push(Message {
          author: Author::Assistent,
          content: answer
        })
      }
    }
  }
}
