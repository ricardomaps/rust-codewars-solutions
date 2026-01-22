use std::collections::HashMap;

struct Cipher {
    decode_mapping: HashMap<char, char>,
    encode_mapping: HashMap<char, char>,
}

impl Cipher {
    fn new(map1: &str, map2: &str) -> Cipher {
        let mapping = HashMap::from_iter(map1.chars().zip(map2.chars()));
        Cipher {
            decode_mapping: mapping.iter().map(|(&k, &v)| (v, k)).collect(),
            encode_mapping: mapping,
        }
    }

    fn encode(&self, string: &str) -> String {
        String::from_iter(
            string
                .chars()
                .map(|ch| *self.encode_mapping.get(&ch).unwrap_or(&ch)),
        )
    }

    fn decode(&self, string: &str) -> String {
        String::from_iter(
            string
                .chars()
                .map(|ch| *self.decode_mapping.get(&ch).unwrap_or(&ch)),
        )
    }
}
