fn main() {
    let latin = get_string();     
    let pig_latin = make_pig(latin);
    println!("pig latin: {}", pig_latin);
}

fn get_string() -> String {
    let mut line = String::from(""); 
    println!("Enter a string to be converted..");
    std::io::stdin().read_line(&mut line).unwrap();
    return line; 
}

fn make_pig(latin: String) -> String {
    let v = ['a', 'e', 'i', 'o', 'u']; 
    let mut vowel_starter = false; 
    let mut pig_latin = latin;

    for vowel in v {
        if pig_latin.chars().nth(0).unwrap() == vowel { 
            vowel_starter = true; 
        }
    }

    if vowel_starter == true {
        pig_latin = String::from(&pig_latin[0..(pig_latin.len() - 1)]);
        pig_latin.push_str("hay");
        return pig_latin;
    }

    return consonant_pig_latin(pig_latin); 
}

fn consonant_pig_latin(s: String) -> String {
    let mut pig_latin = s;
    let mut first = String::from(&pig_latin[0..1]);
    
    first.push_str("ay");
    pig_latin = String::from(&pig_latin[1..(pig_latin.len() - 1) ]); 
    pig_latin.push_str(&first); 
    return pig_latin;
}
