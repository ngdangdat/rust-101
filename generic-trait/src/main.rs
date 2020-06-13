fn longest<'a>(x: &'a str, y: &'a str) -> str {
    let tmp;
    if x.len() > y.len() {
        tmp = String::from("first").as_str();
    } else {
        tmp = String::from("second").as_str();
    }

    *tmp
}

fn main() {
    let f = String::from("first");
    let result;

    {
        let string2 = String::from("s");
        result = longest(f.as_str(), string2.as_str());
        println!("Longer string is: {}", result);
    }

}

