use protozero::{Error, Message};

fn list_people(address_book: Message<'_>) -> Result<(), Error> {
    for field in address_book.fields() {
        let field = field?;
        match field.number {
            // repeated Person people = 1;
            1 => {
                let mut name: &str = "";
                let mut id: i32 = 0;
                let mut email: &str = "";
                enum PhoneType {
                    Mobile,
                    Home,
                    Work,
                }
                struct PhoneNumber<'a> {
                    number: &'a str,
                    type_: Option<PhoneType>,
                }
                let mut phones: Vec<PhoneNumber> = Vec::new();

                for field in field.value.get_message()?.fields() {
                    let field = field?;
                    match field.number {
                        // string name = 1;
                        1 => name = field.value.get_string()?,
                        // int32 id = 2;
                        2 => id = field.value.get_int32()?,
                        // string email = 3;
                        3 => email = field.value.get_string()?,
                        // repeated PhoneNumber phones = 4;
                        4 => {
                            let mut number: &str = "";
                            let mut type_: Option<PhoneType> = Some(PhoneType::Mobile);

                            for field in field.value.get_message()?.fields() {
                                let field = field?;
                                match field.number {
                                    // string number = 1;
                                    1 => number = field.value.get_string()?,
                                    // PhoneType type = 2;
                                    2 => {
                                        type_ = match field.value.get_enum()? {
                                            // MOBILE = 0;
                                            0 => Some(PhoneType::Mobile),
                                            // HOME = 1;
                                            1 => Some(PhoneType::Home),
                                            // WORK = 2;
                                            2 => Some(PhoneType::Work),
                                            _ => None,
                                        };
                                    }
                                    _ => {}
                                }
                            }
                            phones.push(PhoneNumber { number, type_ })
                        }
                        _ => {}
                    }
                }

                println!("Person ID: {}", id);
                println!("  Name: {}", name);
                if !email.is_empty() {
                    println!("  E-mail address: {}", email);
                }
                for phone in &phones {
                    match phone.type_ {
                        Some(PhoneType::Mobile) => println!("  Mobile phone #: {}", phone.number),
                        Some(PhoneType::Home) => println!("  Home phone #: {}", phone.number),
                        Some(PhoneType::Work) => println!("  Work phone #: {}", phone.number),
                        None => println!("  Unknown phone #: {}", phone.number),
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

use std::io;
use std::process::exit;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    if args.len() != 2 {
        eprintln!("Usage: {} ADDRESS_BOOK_FILE", args.nth(0).unwrap());
        exit(1);
    }
    let path = args.nth(1).unwrap();
    let buf = std::fs::read(path)?;

    if let Err(_) = list_people(Message::new(&buf)) {
        eprintln!("Failed to parse address book.");
        exit(1);
    }
    Ok(())
}
