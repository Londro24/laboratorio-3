use std::io::stdin;

fn conversion(adn_copy:String) -> String{
    let mut arn: String = String::new();
    for c in adn_copy.to_string().trim().chars(){
        if c == 'A' {
            arn = arn + "U";
        } else if c == 'T' {
            arn = arn + "A";
        } else if c == 'C' {
            arn = arn + "G";
        } else if c == 'G' {
            arn = arn + "C";
        }
    }
    return arn;
}

fn main(){
    let mut adn: String = String::new();
    let mut adn_copy: String = String::new();
    let mut corrupto: bool = false;
    stdin().read_line(&mut adn).unwrap();
    for c in adn.to_string().trim().chars(){
        if c == 'A' || c == 'T' || c == 'C' || c == 'G' {
            adn_copy = adn_copy + &c.to_string();
        } else if c == 'a' {
            adn_copy = adn_copy + "A";
        } else if c == 'c' {
            adn_copy = adn_copy + "C";
        } else if c == 't' {
            adn_copy = adn_copy + "T";
        } else if c == 'g' {
            adn_copy = adn_copy + "G";
        } else {
            corrupto = true
        }
    }

    println!("Desde su codigo {}", adn_copy);

    let arn:String = conversion(adn_copy);
    println!("Consiguio la cadena {}", arn);
    if corrupto == true {
        println!("Su codigo gentico esta corrupto, revislo por si acaso")
    }
}