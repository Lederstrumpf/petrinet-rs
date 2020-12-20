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
    let regex_trait_name = r"(?:trait )([^<]+)";
    let regex_produce = r"(?:\bProduce<)([^>]+)";
    let regex_consume = r"(?:\bConsume<)([^>]+)";
    let regex_consume_secondary = r"(?:, )?([^:,<]+):( Produce<)";
    let regex_produce_secondary = r"(?:, )?([^:,<]+):( Consume<)";

    fn extract_produce(line: &String, regex_string: &str) -> Vec<String> {
        let re = Regex::new(regex_string).unwrap();
        let group = match re.captures(line).unwrap() {
            Some(group) => extract_from_tuple(
                &group
                    .get(1)
                    .map_or(None, |m| Some(m.as_str().to_string()))
                    .unwrap(),
            ),
            None => vec![],
        };
        group
    }

    fn extract_from_tuple(value: &String) -> Vec<String> {
        let re = Regex::new(r"\(.*\)").unwrap();
        match re.is_match(value) {
            Ok(true) => value[1..value.len() - 1]
                .split(", ")
                .map(|sec| sec.to_string())
                .collect::<Vec<String>>(),
            Ok(false) => vec![value.to_string()],
            Err(_) => vec![value.to_string()],
        }
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
            .map(|line| line.unwrap())
            .collect::<std::vec::Vec<std::string::String>>();

        #[derive(Debug)]
        struct Extract {
            line: String,
            trait_name: Vec<String>,
            produce: Vec<String>,
            produce_secondary: Vec<String>,
            consume: Vec<String>,
            consume_secondary: Vec<String>,
        }

        let result = petrinet_traits
            .iter()
            .map(|line| Extract {
                line: line.to_string(),
                trait_name: extract_produce(line, regex_trait_name),
                produce: extract_produce(line, regex_produce),
                produce_secondary: extract_produce(line, regex_produce_secondary),
                consume: extract_produce(line, regex_consume),
                consume_secondary: extract_produce(line, regex_consume_secondary),
            })
            .collect::<Vec<Extract>>();

        fn build_line(extract: &Extract) -> String {
            let res = match (extract.produce.len(), extract.consume.len()) {
                (0, 0) => (extract.produce_secondary.clone(), extract.consume_secondary.clone()),
                (_, 0) => (extract.produce.clone(), extract.consume_secondary.clone()),
                (0, _) => (extract.produce_secondary.clone(), extract.consume.clone()),
                (_, _) => (extract.produce.clone(), extract.consume.clone()),
            };
            println!("{:?} consists of:\n{:?}", extract.trait_name, res);
            "".to_string()
        }

        result.iter().for_each(|line| {build_line(line);});
    }
}
