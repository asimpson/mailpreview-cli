use mailparse::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;

fn return_body_from_alternative(mail: &ParsedMail, format: &String) -> Result<String, MailParseError> {
    let mut body = String::new();

    for m in mail.subparts.iter() {
      if &m.ctype.mimetype == format {
        body = m.get_body()?;
      }
    }

    Ok(body)
}

fn return_body(mail: ParsedMail, format: String) -> Result<String, MailParseError> {
    let mut body = String::new();

    if mail.subparts.len() > 0 {
        for m in mail.subparts.iter() {
            if m.ctype.mimetype == "multipart/related" {
              // TODO make alternative extractor func
              // TODO account for mixed
              for i in m.subparts.iter() {
                if i.ctype.mimetype == "multipart/alternative" {
                  body = return_body_from_alternative(i, &format)?;
                }
              }
            }
            if m.ctype.mimetype == "multipart/alternative" {
                body = return_body_from_alternative(m, &format)?;
            }
            if m.ctype.mimetype == format {
                body = m.get_body()?;
            }
        }
    } else {
        body = mail.get_body()?;
    }

    if body.len() == 0 {
      println!("No processable body in email");
      std::process::exit(1);
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

fn return_format_from_cli() -> String {
    let format = std::env::args().nth(2);

    if format == Some("text/html".to_string()) {
      return "text/html".to_string()
    } else {
      return "text/plain".to_string()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = return_path_from_cli()?;
    let format = return_format_from_cli();
    let mut contents = String::new();
    File::open(file.trim())?.read_to_string(&mut contents)?;

    let mail = parse_mail(contents.as_bytes())?;
    let message_id = mail
        .headers
        .get_first_value("Message-ID")
        .expect("Message-ID");
    let file_name = format!("/tmp/{}", message_id.replace("/", "-"));
    let body = return_body(mail, format).expect("parsed body");

    File::create(&file_name)?.write_all(body.as_bytes())?;
    println!("{}", &file_name);

    Ok(())
}
