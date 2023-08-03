#[derive(Debug)]
pub struct Person {
    id: u32,
    pub name: String,
    pub birth_year: u32,
}

#[derive(Debug, Default)]
pub struct Class {
    pub id: u8,
    pub name: String,
    pub capacity: u8,
}

impl Person {
    pub fn new(name: String, birth_year: u32) -> Result<Self, String> {
        if birth_year < 1970 {
            Err("birth date is invalid".to_string())
        } else {
            Ok(Person {
                id: 0,
                name,
                birth_year,
            })
        }
    }
}

impl Default for Person {
    fn default() -> Self {
        Person {
            id: 0,
            name: "N/A".to_string(),
            birth_year: 1970,
        }
    }
}
