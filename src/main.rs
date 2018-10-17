extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_male: bool,
}

impl Person {
    fn get_gender(&self) -> String {
        if self.is_male {
            return String::from("male");
        } else {
            return String::from("female");
        }
    }
}

fn main() {
    let json_str = r#"
    {
        "name" : "David",
        "age" : 29,
        "is_male" : true
    }
    "#;

    let res = serde_json::from_str(json_str);

    if res.is_ok() {
        let p: Person = res.unwrap();
        println!("{} is a {} years old {}", p.name, p.age, p.get_gender());
    } else {
        println!("Error: Could not parse JSON.");
    }
}
