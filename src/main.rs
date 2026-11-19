use std::collections::HashMap;

struct TrieNode {
    is_node: bool,
    nei: Vec<TrieNode>,
}

fn build_trie(_obj_n_label_set: Vec<(String, Vec<u32>)>) {
    todo!();
}

fn build_freq_map(obj_labels: &Vec<Vec<String>>) -> HashMap<&String, u32> {
    let mut label_freq_map: HashMap<&String, u32> = HashMap::new();

    for obj_n_labels in obj_labels.iter() {
        let mut it = obj_n_labels.iter();

        // skip the obj
        it.next();

        for label in it {
            match label_freq_map.get_mut(label) {
                Some(freq) => *freq += 1,
                None => {
                    label_freq_map.insert(label, 1);
                }
            }
        }
    }

    label_freq_map
}

fn sort_label_by_freq<'a>(label_freq_map: &HashMap<&'a String, u32>) -> Vec<(&'a String, u32)> {
    let mut label_freq_vec: Vec<(&String, u32)> = Vec::new();
    label_freq_vec.reserve(label_freq_map.len());

    for (label, freq) in label_freq_map.iter() {
        label_freq_vec.push((label, *freq));
    }
    label_freq_vec.sort_by(|(_, fa), (_, fb)| fb.cmp(fa));

    label_freq_vec
}

fn generate_label_ids<'a>(label_freq_vec: &Vec<(&'a String, u32)>) -> HashMap<&'a String, u32> {
    let mut next_id: u32 = 0;
    let mut label_id_map: HashMap<&'a String, u32> = HashMap::new();
    for (label, _) in label_freq_vec {
        label_id_map.insert(label, next_id);
        next_id += 1;
    }

    label_id_map
}

fn build_obj_n_label_id_set(
    obj_labels: &Vec<Vec<String>>,
    label_id_map: &HashMap<&String, u32>,
) -> Vec<(String, Vec<u32>)> {
    let mut obj_n_label_set: Vec<(String, Vec<u32>)> = Vec::new();
    for obj_n_labels in obj_labels.iter() {
        let mut it = obj_n_labels.iter();

        let Some(obj) = it.next() else {
            continue;
        };

        let mut label_set: Vec<u32> = Vec::new();
        for label_str in it {
            if let Some(&label_id) = label_id_map.get(label_str) {
                label_set.push(label_id);
            };
        }

        obj_n_label_set.push((obj.clone(), label_set));
    }

    obj_n_label_set.sort_by(|(_, label_set_a), (_, label_set_b)| label_set_a.cmp(label_set_b));

    obj_n_label_set
}

fn main() {
    let obj_labels: Vec<Vec<String>> = Vec::new();

    let label_freq_map = build_freq_map(&obj_labels);

    let label_freq_vec = sort_label_by_freq(&label_freq_map);

    let label_id_map = generate_label_ids(&label_freq_vec);

    let obj_n_label_set = build_obj_n_label_id_set(&obj_labels, &label_id_map);

    build_trie(obj_n_label_set);
}
