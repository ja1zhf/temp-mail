use temp_mail::Email;

const GRAY: &str = "\x1b[30;1m";
const RESET: &str = "\x1b[0m";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut email = Email::new().await?;
    let mut count = 0;

    println!(
        "{}{}\n",
        email.address,
        format!("{}(already copied!){}", GRAY, RESET)
    );

    loop {
        email.get_messages().await?;
        if email.messages.len() > count {
            email.print_messages();
            count += 1;
        }
    }
}
