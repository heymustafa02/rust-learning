fn main() {
    let str1 = String::from("Mustafa");
    let str2 = String::from("Mallebhari");

    let ans = longest_string(&str1, &str2);

    println!("Longest String is: {}", ans);
}

fn longest_string<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
