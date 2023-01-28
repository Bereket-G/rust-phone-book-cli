use std::fs::File;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Contact {
    phone_number: String,
    name: String,
}

pub struct PhoneBook {
    pub phone_book: Vec<Contact>,
    pub db_file_name: String
}


impl PhoneBook {
    pub fn new(contact_file_name: &str) -> Self {
        let file: File = File::open(contact_file_name)
            .expect("file should open read only");
        PhoneBook {
            phone_book: serde_json::from_reader(file).unwrap(),
            db_file_name: String::from(contact_file_name)
        }
    }

    pub fn get_contact(self, name: String) -> Contact {
        let contact = self.phone_book.iter().find(|x| x.name == name).unwrap();
        return contact.clone();
    }

    pub fn add_contact(&mut self, name: String, phone_number: String) {
        self.phone_book.push( Contact {
            name,
            phone_number
        } );
        std::fs::write(
            &self.db_file_name,
            serde_json::to_string_pretty(&self.phone_book).unwrap(),
        ).expect("Unable to write to file");
    }
}
