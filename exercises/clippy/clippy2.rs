//DONE

fn main() {
    let mut res = 42;
    let option = Some(12);
    for x in option.iter() {
        res += x;
    }
    println!("{}", res);
}
