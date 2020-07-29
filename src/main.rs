use mailparse::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;

fn return_body(mail: ParsedMail) -> Result<String, MailParseError> {
    let mut body = String::new();

    if mail.subparts.len() > 0 {
        for m in mail.subparts.iter() {
            if m.ctype.mimetype == "text/plain" {
                body = m.get_body()?;
            }
        }
    } else {
        body = mail.get_body()?;
    }

    Ok(body)
}

fn return_path_from_cli() -> Result<String, String> {
    let file = std::env::args().nth(1);

    match file {
        Some(f) => Ok(f),
        None => Err("No message file was provided.".to_string()),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = return_path_from_cli()?;
    let mut contents = String::new();
    File::open(file.trim())?.read_to_string(&mut contents)?;

    let mail = parse_mail(contents.as_bytes())?;
    let message_id = mail
        .headers
        .get_first_value("Message-ID")
        .expect("Message-ID");
    let file_name = format!("/tmp/{}", message_id);
    let body = return_body(mail).expect("parsed body");

    File::create(&file_name)?.write_all(body.as_bytes())?;
    println!("{}", &file_name);

    Ok(())
}
