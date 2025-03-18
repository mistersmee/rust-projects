enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn vectors() {
    let _v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3, 4, 5];

    let mut v1 = Vec::new();
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v2 = vec![100, 32, 57];

    for i in &v2 {
        println!("{i}");
    }

    let mut v3 = vec![100, 32, 57] ;

    for i in &mut v3 {
        *i += 50;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
