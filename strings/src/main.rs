fn main() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    //string slices and strings are utf-8 encoded
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    s.push('j');

    let c1 = String::from("Hello, ");
    let c2 = String::from("world!");
    let s3 = c1 + &c2;//note c1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); //doesn't take ownership of any of the strings

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    //above can crash the program if not done carefullty
    
    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }
}
