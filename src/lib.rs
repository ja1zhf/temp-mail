use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use dialoguer::{theme::ColorfulTheme, Select};
use serde::Deserialize;

const API: &str = "https://www.1secmail.com/api/v1/";

#[derive(Deserialize, Debug)]
struct GetEmail {
    email: String,
}

#[derive(Deserialize, Debug)]
pub struct MessagePreview {
    id: usize,
    from: String,
    subject: String,
    date: String,
}

#[derive(Debug)]
pub struct Email {
    pub address: String,
    pub messages: Vec<MessagePreview>,
}

impl Email {
    pub async fn new() -> Result<Self, reqwest::Error> {
        let request = reqwest::get(format!("{}?action=genRandomMailbox&count=1", API))
            .await?
            .text()
            .await?;

        let data: GetEmail = serde_json::from_str(&request).unwrap();

        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(data.email.clone()).unwrap();

        Ok(Email {
            address: data.email,
            messages: Vec::new(),
        })
    }

    pub async fn get_messages(&mut self) -> Result<(), reqwest::Error> {
        let auth: Vec<&str> = self.address.split('@').collect();
        let request = reqwest::get(format!(
            "{}?action=getMessages&login={}&domain={}",
            API, auth[0], auth[1]
        ))
        .await?
        .text()
        .await?;

        let data: Vec<MessagePreview> = serde_json::from_str(&request).unwrap();

        self.messages = data;

        Ok(())
    }

    pub fn print_messages(&self) {
        let mut msgs: Vec<String> = Vec::new();

        for message in &self.messages {
            msgs.push(format!(
                "From: {}\nSubject: {}\nDate: {}\n",
                message.from, message.subject, message.date
            ));
        }

        let selection: usize = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&msgs)
            .interact()
            .unwrap();

        println!("{selection}");
    }
}
