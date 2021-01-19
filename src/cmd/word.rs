pub struct Words {
    words: Vec<String>,
}

impl Words {
    pub fn new(input: &str) -> Self {
        let words = input
            .split(|c| "_- ".contains(c))
            .flat_map(Self::split_camel)
            .filter(|s| !s.is_empty())
            .collect();
        Words { words }
    }

    fn split_camel(input: &str) -> Vec<String> {
        let f = input.chars();
        let s = input.chars().skip(1);
        let split_indices: Vec<usize> = f
            .zip(s)
            .enumerate()
            .filter(|(_, (f, s))| Self::is_boundary(*f, *s))
            .map(|(i, _)| i + 1)
            .collect();

        Self::split_at_indices(input, split_indices)
    }

    fn is_boundary(f: char, s: char) -> bool {
        (f.is_lowercase() && s.is_uppercase())
            || (f.is_ascii_digit() && !(s.is_ascii_punctuation() || s.is_ascii_digit()))
            || (!(f.is_ascii_punctuation() || f.is_ascii_digit()) && s.is_ascii_digit())
    }

    fn split_at_indices(input: &str, indices: Vec<usize>) -> Vec<String> {
        let mut words = Vec::new();
        let mut tmp = input;
        for &x in indices.iter().rev() {
            let pair = tmp.split_at(x);
            tmp = pair.0;
            words.push(pair.1);
        }
        words.push(tmp);

        words.iter().rev().map(ToString::to_string).collect()
    }

    pub fn into_case(mut self, case: &str) -> String {
        match case.to_lowercase().as_str() {
            "camel" => {
                self.make_camel_case();
                self.join("")
            }
            "pascal" => {
                self.capitalize();
                self.join("")
            }
            "snake" => {
                self.make_lowercase();
                self.join("_")
            }
            "uppersnake" => {
                self.make_uppercase();
                self.join("_")
            }
            "kebab" => {
                self.make_lowercase();
                self.join("-")
            }
            "cobol" => {
                self.make_uppercase();
                self.join("-")
            }
            "lower" => {
                self.make_lowercase();
                self.join("")
            }
            "upper" => {
                self.make_uppercase();
                self.join("")
            }
            &_ => String::from("Not support!"),
        }
    }

    fn make_camel_case(&mut self) {
        self.words = self
            .words
            .iter()
            .enumerate()
            .map(|(i, word)| {
                if i != 0 {
                    let mut chars = word.chars();
                    if let Some(a) = chars.next() {
                        a.to_uppercase()
                            .chain(chars.as_str().to_lowercase().chars())
                            .collect()
                    } else {
                        String::new()
                    }
                } else {
                    word.to_lowercase()
                }
            })
            .collect();
    }

    fn capitalize(&mut self) {
        self.words = self
            .words
            .iter()
            .map(|word| {
                let mut chars = word.chars();
                if let Some(a) = chars.next() {
                    a.to_uppercase()
                        .chain(chars.as_str().to_lowercase().chars())
                        .collect()
                } else {
                    String::new()
                }
            })
            .collect();
    }

    fn make_lowercase(&mut self) {
        self.words = self.words.iter().map(|word| word.to_lowercase()).collect()
    }

    fn make_uppercase(&mut self) {
        self.words = self.words.iter().map(|word| word.to_uppercase()).collect()
    }

    fn join(self, delim: &str) -> String {
        self.words
            .iter()
            .enumerate()
            .map(|(i, val)| {
                if i == 0 {
                    val.to_owned()
                } else {
                    delim.to_owned() + val
                }
            })
            .collect()
    }
}
