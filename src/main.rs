use temp_mail::Email;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut email = Email::new().await?;

    println!("{}(already copied!)", email.address);

    loop {
        email.get_messages().await?;
        email.print_messages();
    }

    Ok(())
}
