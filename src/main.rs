use std::io;

fn main() {
    let mut in_str = String::new();
    println!("Enter line:");
    match io::stdin().read_line(&mut in_str) {
        Ok(_) => {
            println!("The longest streak is: {}", tt_str_fil::string_filter(&in_str));
        }
        Err(e) => {
            println!("Holymolly, input error! {}", e);
        }
    }
}
