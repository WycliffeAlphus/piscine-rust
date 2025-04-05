use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {

    let mut res :HashMap<&str, usize> = HashMap::new();

    for word in words{
        if res.contains_key(word){
            if let Some(value) = res.get_mut(word){
                *value +=1;
            }
        } else {
            res.insert(word, 1);
        }
        
    }
res
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {

    let mut uniq = 0;

    for (_, v) in frequency_count.iter(){
        if *v as u32 == 1{
            uniq += 1;
        }
    }
uniq    
}
