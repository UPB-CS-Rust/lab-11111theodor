//! Make me compile without adding new lines-- just changing existing lines!

fn main() {
    let s0 = String::new();

    let mut s1 = append_to_string(s0.clone()); // in loc de s0 facem so.clone

    println!("{} == `{}`", stringify!(s1), s1);

    s1.push('!');

    println!("{} == `{}`", stringify!(s1), s1);
}

fn append_to_string(mut s: String) -> String { // si am facut s: string de tip mut 
    s.push_str("Hello World");

    s
}
