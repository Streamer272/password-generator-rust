use rand::Rng;

fn main() {
    let length = 16;
    let mut generated = String::new();
    let include_lowercase = true;
    let include_uppercase = true;
    let include_numbers = true;
    let include_special = true;

    let mut characters = Vec::<char>::new();
    if include_lowercase {
        for it in 'a'..='z' {
            characters.push(it)
        }
    }
    if include_uppercase {
        for it in 'A'..='Z' {
            characters.push(it)
        }
    }
    if include_numbers {
        for it in '0'..='9' {
            characters.push(it)
        }
    }
    if include_special {
        for it in '!'..='/' {
            characters.push(it)
        }
    }

    let mut thread = rand::thread_rng();
    for _ in 0..length {
        let random = thread.gen_range(0..characters.len());
        generated.push(characters[random]);
    }

    println!("{}", generated);
}
