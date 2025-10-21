use std::collections::HashMap;

const TEXT: &str = "aaasda sdafsdgh ssaaas aaasda djahfsldfa";

fn new_word<'a>(map: &mut HashMap<&'a str, Vec<usize>>, word: &'a str, index: usize)
{
    if let Some(vector) = map.get_mut(word) {
        vector.push(index);
        println!("duplicate word {} {:?}", word, vector);
    }
    else {
        map.insert(word, Vec::from([index]));
    }

}

fn index_words(_text: &str) -> HashMap<&str, Vec<usize>> {
    let mut map: HashMap<&str, Vec<usize>> = HashMap::new();

    let bytes = _text.as_bytes();
    let mut spaceindex = 0;

    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' { 
            let word = &_text[spaceindex..i];
            new_word(& mut map, word, spaceindex);
            spaceindex = i + 1;

        }
    }
    new_word(& mut map, &_text[spaceindex..], spaceindex);
    
    return map;
}

fn main() {
//    println!("Hello, world!");
    let map = index_words(TEXT);
    println!("{map:#?}");
}
