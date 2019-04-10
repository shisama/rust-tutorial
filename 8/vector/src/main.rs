fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:?}", vec);

    useMacro();
    iterate();
    enumerate();
}

fn useMacro() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("third: {:?}", third);

    let third: Option<&i32> = v.get(2);
    println!("optional third: {:?}", third);
}

fn iterate() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn enumerate() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        println!("{:?}", i);
    }
}