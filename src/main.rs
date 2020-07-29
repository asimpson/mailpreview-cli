use mailparse::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;

fn return_body(mail: ParsedMail) -> Result<String, MailParseError> {
    let mut body = "".to_string();

    if mail.subparts.len() > 0 {
        for m in mail.subparts.iter() {
            if m.ctype.mimetype == "text/plain" {
                body = m.get_body()?;
            }
        }
    } else {
        body = mail.get_body()?;
    }

    return Ok(body);
}

fn main() {
    let file = std::env::args().nth(1).expect("mail message to parse");
    let mut file_handle = File::open(file.trim()).expect("file handle to contents of message");
    let mut contents = String::new();

    file_handle
        .read_to_string(&mut contents)
        .expect("actual message content");
    let mail = parse_mail(contents.as_bytes()).expect("parsed mail struct");
    let message_id = mail
        .headers
        .get_first_value("Message-ID")
        .expect("Message-ID");
    let file_name = format!("/tmp/{}", message_id);
    let mut tmp_file = File::create(&file_name).expect("tmp file created");
    let body = return_body(mail).expect("parsed body");


    println!("{}", &file_name);
    tmp_file.write_all(body.as_bytes()).expect("file created");
}
