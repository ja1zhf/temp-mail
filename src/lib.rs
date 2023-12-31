use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use dialoguer::{theme::ColorfulTheme, Select};
use scraper::{Html, Selector};
use serde::Deserialize;

mod colors;
pub use colors::{GRAY, RESET};

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

#[derive(Deserialize, Debug)]
pub struct Attachment {
    filename: String,
    content_type: String,
    size: usize,
}

#[derive(Deserialize, Debug)]
pub struct MessageContant {
    id: usize,
    from: String,
    subject: String,
    date: String,
    attachments: Vec<Attachment>,
    body: String,
    textBody: String,
    htmlBody: String,
}

#[derive(Debug)]
pub struct Email {
    pub address: String,
    pub messages: Vec<MessagePreview>,
}

impl Email {
    pub async fn new() -> Result<Self, reqwest::Error> {
        let response = reqwest::get(format!("{}?action=genRandomMailbox&count=1", API))
            .await?
            .text()
            .await?;

        let data: GetEmail = serde_json::from_str(&response).unwrap();

        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(data.email.clone()).unwrap();

        Ok(Email {
            address: data.email,
            messages: Vec::new(),
        })
    }

    pub async fn get_messages(&mut self) -> Result<(), reqwest::Error> {
        let auth: Vec<&str> = self.address.split('@').collect();
        let response = reqwest::get(format!(
            "{}?action=getMessages&login={}&domain={}",
            API, auth[0], auth[1]
        ))
        .await?
        .text()
        .await?;

        self.messages = serde_json::from_str(&response).unwrap();

        Ok(())
    }

    async fn get_message(&mut self, id: usize) -> Result<MessageContant, reqwest::Error> {
        let auth: Vec<&str> = self.address.split('@').collect();
        let response = reqwest::get(format!(
            "{}?action=readMessage&login={}&domain={}&id={}",
            API, auth[0], auth[1], id
        ))
        .await?
        .text()
        .await?;

        let data: MessageContant = serde_json::from_str(&response).unwrap();

        Ok(data)
    }

    pub async fn print_messages(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut msgs: Vec<String> = self
            .messages
            .iter()
            .map(|message| {
                format!(
                    "From: {}\nSubject: {}\nDate: {}\n",
                    message.from, message.subject, message.date
                )
            })
            .collect();

        msgs.push("Reload".to_string());

        let selection: usize = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&msgs)
            .interact()
            .unwrap();

        if selection != msgs.len() - 1 {
            let data = self.get_message(self.messages[selection].id).await?;

            if data.textBody.len() != 0 {
                println!(
                    "From: {GRAY}{}{RESET} Subject: {GRAY}{}{RESET}, Date: {GRAY}{}{RESET}\n\n{}",
                    data.from, data.subject, data.date, data.textBody
                );
            } else {
                let html = Html::parse_document(&data.htmlBody);
                let selector = Selector::parse("a").unwrap();
                for element in html.select(&selector) {
                    let text = element.text().collect::<Vec<_>>();
                    let hrefs = element.value().attr("href").unwrap();
                    println!("{}: {}", text.join(" "), hrefs);
                }
            }

            Select::with_theme(&ColorfulTheme::default())
                .default(0)
                .items(&["Exit"])
                .interact()
                .unwrap();

            print!("\x1B[2J\x1B[1;1H");
            println!(
                "{}{}\n",
                self.address,
                format!("{}(already copied!){}", GRAY, RESET)
            );
        }

        Ok(())
    }
}
