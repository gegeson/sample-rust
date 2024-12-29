fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x); // xの値は{}です
    x = 6;
    println!("The value of x is: {}", x);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);
    for element in a {
        println!("The value of element is: {}", element);
    }

    another_function(x);

    let s = String::from("hello");

    takes_ownership(&s);

    let x = 5; // xがスコープに入る

    makes_copy(x);

    println!("The value of x is: {}", x); // xの値は{}です

    let s = String::from("hello world");

    let word = first_word(&s);

    println!("The first word is: {}", word);

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    // first_wordは`String`のスライスに対して機能する
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    // first_wordは文字列リテラルのスライスに対して機能する
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    // 文字列リテラルは「それ自体すでに文字列スライスなので」、
    // スライス記法なしでも機能するのだ！
    let word = first_word(my_string_literal);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn takes_ownership(some_string: &String) {
    // some_stringがスコープに入る。
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    // some_integerがスコープに入る
    println!("{}", some_integer);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
