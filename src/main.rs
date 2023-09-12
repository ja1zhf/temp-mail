use colored::Colorize;
use temp_mail::Email;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut email = Email::new().await?;
    let mut count = 0;

    println!("{}{}\n", email.address, "(already copied!)".cyan());

    loop {
        email.get_messages().await?;
        if email.messages.len() > count {
            email.print_messages();
            count += 1;
        }
    }
}
