use banner::print_banner;
use std::mem;

#[derive(Debug)]
enum Package {
    Free{name:String},
    Basic{name:String},
    Pro{name:String}
}

fn upgrade_package(package: &mut Package) {
    use Package::*;

    *package  = match package {
        Free { name } => Basic { name: mem::take(name)},
        Basic { name } => Pro {name: mem::take(name)},
        Pro { name: _ } => return,
    };

}


fn main() {
    print_banner();

    let mut f = Package::Free {name:"Ramazan".to_string()};
    println!("current package: {:?}",f);
    upgrade_package(&mut f);
    println!("1. after upgrade package: {:?}",f);
    upgrade_package(&mut f);
    println!("2. after upgrade package: {:?}",f);
    upgrade_package(&mut f);
    println!("3. after upgrade package: {:?}",f);
    upgrade_package(&mut f);
    println!("4. after upgrade package: {:?}",f);
    upgrade_package(&mut f);
    println!("5. after upgrade package: {:?}",f);


    let mut first = "go".to_string();
    let mut second = "rust".to_string();


    println!("first: {}, second: {}",first,second);

    // long version
    // let java = first;
    // first = second;
    // second = java;
    mem::swap(&mut first, &mut second);

    println!("after swap first: {}, second: {}",first,second);

}
