#[test]
fn extract_petrinet() {
    // use regex::Regex;
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

    let regex_trait_header = Regex::new(r"// petrinet definition").unwrap();
    let regex_trait = Regex::new(r"trait").unwrap();
    let regex_produce_and_consume = Regex::new(r"(?=.*\bProduce\b)(?=.*\bConsume\b).*").unwrap();
    let regex_trait_name = r"(?:trait )([^<]+)";
    let regex_produce = r"(?:\bProduce<)([^>]+)";
    let regex_consume = r"(?:\bConsume<)([^>]+)";
    let regex_consume_secondary = r", ([^:,]+):( Produce<)";
    let regex_produce_secondary = r", ([^:,]+):( Consume<)";

    fn extract_produce(line: &String, regex_string: &str) -> Option<String> {
        // println!("Line to extract produce from: {}", line);
        let re = Regex::new(regex_string).unwrap();
        let group = match re.captures(line).unwrap() {
            Some(group) => group.get(1).map_or(None, |m| Some(m.as_str().to_string())),
            None => None,
        };
        println!(
            "Extracted from {:?}\nusing {:?}:\n{:?}\n",
            line,
            re,
            extract_from_tuple(group.as_ref().unwrap_or(&"".to_string()))
        );
        group
    }

    fn extract_from_tuple(value: &String) -> Vec<String> {
        let re = Regex::new(r"\((.*)\)").unwrap();
        match re.is_match(value) {
            Ok(true) => value[1..value.len() - 1]
                .split(", ")
                .map(|sec| sec.to_string())
                .collect::<Vec<String>>(),
            Ok(false) => vec![value.to_string()],
            Err(_) => vec![value.to_string()],
        }
        // let re = Regex::new(r"(\w+(?:, )*)+").unwrap();
        // let re = Regex::new(r"\w*").unwrap();
        // match re.captures(value).unwrap_or(None) {
        //     Some(group) => group
        //         .iter()
        //         // .skip(1)
        //         .map(|element| match element {
        //             Some(el) => el.as_str().to_string(),
        //             None => "".to_string()
        //         })
        //         .collect(),
        //     None => vec![],
        // }
    }

    if let Ok(lines) = read_lines("./src/protocol2.rs") {
        let petrinet_traits = lines
            .skip_while(|line| {
                !regex_trait_header
                    .is_match(&line.as_ref().unwrap())
                    .unwrap()
            })
            .skip(1)
            .take_while(|line| regex_trait.is_match(&line.as_ref().unwrap()).unwrap())
            // .filter(|line| {
            //     regex_produce_and_consume
            //         .is_match(&line.as_ref().unwrap())
            //         .unwrap()
            // })
            .map(|line| line.unwrap())
            .collect::<std::vec::Vec<std::string::String>>();

        let result = petrinet_traits
            .iter()
            .map(|line| {
                (
                    line,
                    extract_produce(line, regex_trait_name),
                    extract_produce(line, regex_produce),
                    extract_produce(line, regex_produce_secondary),
                    extract_produce(line, regex_consume),
                    extract_produce(line, regex_consume_secondary),
                )
            })
            .collect::<Vec<(
                &String,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
            )>>();
        result.iter().for_each(|line| println!("line: {:?}", line));
    }
}
