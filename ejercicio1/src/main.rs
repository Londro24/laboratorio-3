use std::io::stdin;

fn is_isbn10(clean_isbn_copy: String) -> bool{
    // primero se crean 2 variables bases para las operaciones de verficado
    let mut sum:u32 = 0;
    let mut contador:u32 = 10;
    for d in clean_isbn_copy.to_string().chars() {
        if d == 'X' || d == 'x' {
            let d: u32 = 10;
            sum = sum + d*contador;
        } else if d == '1' || d == '2' || d == '3' || d == '4'|| d == '5'|| 
        d == '6'|| d == '7'|| d == '8'|| d == '9'|| d == '0'{
            let d: u32 = d as u32 - '0' as u32;
            sum = sum + d*contador;
        }
        contador = contador - 1;
    }
    if sum % 11 == 0 {
        return true;
    } else {
        return false;
    }
}


fn is_isbn_format_valid(c: &str) -> bool {
    // lo que hace es validar que sean numeros o una x para que de esa manera sean insertados en la variable
    if c.chars().next().unwrap().is_numeric() {
        return true;
    } else if c == "X" || c == "x"{
        return true;
    }
    return false
}


fn main(){
    let mut isbn: String = String::new();
    let mut clean_isbn: String = String::new();
    let mut clean_isbn_copy: String = String::new();
    stdin().read_line(&mut isbn).unwrap();
   
    // Ambos ciclos lo que hacen es formar un ISBN siempre y cuando sea un numero o una "X"
    for c in isbn.to_string().trim().chars(){
        if is_isbn_format_valid(&c.to_string()){
            clean_isbn = clean_isbn + &c.to_string(); 
        }
    }
    for c in isbn.to_string().trim().chars(){
        if is_isbn_format_valid(&c.to_string()){
            clean_isbn_copy = clean_isbn_copy + &c.to_string(); 
        }
    }

    println!("{}", clean_isbn);
    // dependiendo del bool de la funcion
    if is_isbn10(clean_isbn_copy){
        println!("{} es un ISBN10 valido", clean_isbn); //caso verdadero
    } else {
        println!("No es un ISBN10"); // caso falso
    }
}