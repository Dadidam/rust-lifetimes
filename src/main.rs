fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true;
        }
    }

    languages.last().unwrap()
}

fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() >= lang_b.len() {
        lang_a
    } else {
        lang_b
    }
}

fn main() {
    let languages = vec![
        String::from("Rust"),
        String::from("Go"),
        String::from("TypeScript"),
    ];

    let result = next_language(&languages, "Rust");
    println!("{}", result);

    let last_lang = last_language(&languages);
    println!("{}", last_lang);

    let longest_lang = longest_language("Rust", "Go");
    println!("{}", longest_lang);
}
