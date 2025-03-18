
pub fn strings() {
    let mut s = String::new();

    let data = "initial contents";

    let s1 = data.to_string();

    let s2 = "initial contents".to_string();

    let s3 = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s4 = String::from("foo");
    s4.push_str("bar");

    let mut s5 = String::from("foo");
    let s6 = "bar";
    s5.push_str(s6);
    println!("s6 is {s6}");

    let mut s7 = String::from("lo");
    s7.push('l');

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}");
    }

    {
        let hello = String::from("Здравствуйте");
        let s = &hello[0..4];

        for c in "Зд".chars() {
            println!("{c}");
        }

        for b in "Зд".bytes() {
            println!("{b}");
        }
    }
}
