use rand::Rng;

fn add_typo(name: &str) -> String {
    if name.is_empty() {
        return name.to_string();
    }

    let mut rng = rand::thread_rng();
    let mut chars: Vec<char> = name.chars().collect();

    if chars.len() < 2 {
        return name.to_string();
    }

    let typo_type = rng.gen_range(0..3);
    match typo_type {
        0 => {
            let pos = rng.gen_range(0..chars.len());
            chars.insert(pos, chars[pos]);
        },
        1 => {
            if chars.len() >= 2 {
                let pos = rng.gen_range(0..chars.len()-1);
                chars.swap(pos, pos + 1);
            }
        },
        _ => {
            if chars.len() > 1 {
                let pos = rng.gen_range(0..chars.len());
                chars.remove(pos);
            }
        }
    }

    chars.into_iter().collect()
}

fn to_mixed_case(name: &str) -> String {
    name.chars().enumerate().map(|(i, c)| {
        if i % 2 == 0 {
            c.to_uppercase().to_string()
        } else {
            c.to_lowercase().to_string()
        }
    }).collect()
}

fn main() {
    println!("Testing add_typo:");
    for _ in 0..5 {
        println!("  Joshua -> {}", add_typo("Joshua"));
    }
    
    println!("\nTesting to_mixed_case:");
    println!("  Joshua -> {}", to_mixed_case("Joshua"));
    println!("  JOHN -> {}", to_mixed_case("JOHN"));
    println!("  smith -> {}", to_mixed_case("smith"));
}
