use tmail::{Email, GRAY, RESET};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    winconsole::console::set_title("Temp Mail by ja1z + teyllay").unwrap();
    print!("\x1B[2J\x1B[1;1H");
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
