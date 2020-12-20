use fancy_regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn extract_regex(line: &String, regex_string: &str) -> Vec<String> {
    let re = Regex::new(regex_string).expect("Regex is valid");
    match re.captures(line).expect("Regex VM should not crash on regex") {
        Some(group) => extract_elements(
            group
                .get(1)
                .map_or("".to_string(), |m| m.as_str().to_string())
        ),
        None => vec![],
    }
}

fn replace_empty_keyword(value: String) -> String {
    value.replace("Empty", "()")
}

#[derive(Debug)]
struct Extract {
    trait_name: String,
    produce: Vec<String>,
    produce_secondary: Vec<String>,
    consume: Vec<String>,
    consume_secondary: Vec<String>,
}

fn extract_elements(value: String) -> Vec<String> {
    let re = Regex::new(r"\(.*\)").expect("Regex is trivial and valid");
    match re.is_match(&value) {
        Ok(true) => value[1..value.len() - 1]
            .split(", ")
            .map(|sec| replace_empty_keyword(sec.to_string()))
            .collect::<Vec<String>>(),
        Ok(false) => vec![replace_empty_keyword(value)],
        Err(_) => vec![replace_empty_keyword(value)],
    }
}

fn build_line(extract: Extract) -> String {
    let token_tuple = match (extract.produce.len(), extract.consume.len()) {
        (0, 0) => (
            extract.produce_secondary,
            extract.consume_secondary,
        ),
        (_, 0) => (extract.produce, extract.consume_secondary),
        (0, _) => (extract.produce_secondary, extract.consume),
        (_, _) => (extract.produce, extract.consume),
    };
    [
        extract.trait_name,
        ": ".to_string(),
        token_tuple.0.join(" "),
        " -> ".to_string(),
        token_tuple.1.join(" "),
        ".".to_string(),
    ]
    .concat()
}

pub fn main() {
    let regex_trait_header = Regex::new(r"// petrinet definition").expect("Regex is trivial and valid");
    let regex_trait = Regex::new(r"trait").expect("Regex is trivial and valid");
    let regex_trait_name = r"(?:trait )([^<]+)";
    let regex_produce = r"(?:\bProduce<)([^>]+)";
    let regex_consume = r"(?:\bConsume<)([^>]+)";
    let regex_consume_secondary = r"(?:, )?([^:,<]+):( Produce<)";
    let regex_produce_secondary = r"(?:, )?([^:,<]+):( Consume<)";

    if let Ok(lines) = read_lines("./src/protocol2.rs") {
        // Extract the trait definitions from ./src/protocol2.rs
        let petrinet_traits = lines
            .skip_while(|line| {
                !regex_trait_header
                    .is_match(line.as_ref().unwrap_or(&"".to_string()))
                    .expect("Regex VM should not crash on regex")
            })
            .skip(1)
            .take_while(|line| regex_trait.is_match(line.as_ref().unwrap_or(&"".to_string())).expect("Regex VM should not crash on regex"))
            .map(|line| line.unwrap_or("".to_string()))
            .collect::<std::vec::Vec<std::string::String>>();

        // Extract the transition & place names from the traits
        let names = petrinet_traits
            .iter()
            .map(|line| Extract {
                trait_name: extract_regex(line, regex_trait_name).swap_remove(0),
                produce: extract_regex(line, regex_produce),
                produce_secondary: extract_regex(line, regex_produce_secondary),
                consume: extract_regex(line, regex_consume),
                consume_secondary: extract_regex(line, regex_consume_secondary),
            })
            .collect::<Vec<Extract>>();

        // Convert the names into the format used by process.io and print as output
        names
            .into_iter()
            .for_each(|line| println!("{}", build_line(line)));
    }
}
