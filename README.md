# SendEmail

This project was generated with [Rust](https://www.rust-lang.org/).

# Requirements

Configure the following environment variables with data from your SMTP server:
```bash
export USERNAME_EMAIL_RUST="username"
export PASSWORD_EMAIL_RUST="password"
export SMTP_EMAIL_RUST="smtp.mail.com"
```

## Build

Run `cargo build --release` to build the project. The build artifacts will be stored in the `./target/release/` directory.

## Run

Run the binary that was built `./sendmail`

Enter the recipient's address, email title and body, it can be in HTML.