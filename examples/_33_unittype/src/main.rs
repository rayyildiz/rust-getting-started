use banner::print_banner;

fn main() {
    print_banner();


    let name = {
        let a  = "Ramazan Ayyıldız".to_string();
        a.to_uppercase()
    };

    println!("print name : {}",name);

    let s1  = Student {
        name,
        courses: vec!["Rust".to_string(),"Go".to_string()],
        age: 37,
    };

    let n1 = s1.name;
    let c = &s1.courses;
    let age = s1.age;

    //won compile
    //println!("name: {}",s1.name);
    println!("courses: {:?}",s1.courses);
    println!("age: {}",s1.age);

    // wont compile, partially moved
    //println!("student {:?}",s1);
}


#[derive(Debug)]
struct Student {
    name:String,
    courses:Vec<String>,
    age:u8,
}