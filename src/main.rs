use temp_mail::{Email, GRAY, RESET};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut email = Email::new().await?;

    println!(
        "{}{}\n",
        email.address,
        format!("{}(already copied!){}", GRAY, RESET)
    );

    loop {
        email.get_messages().await?;
        email.print_messages().await?;
    }
}
