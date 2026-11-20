use std::collections::HashMap;

#[derive(Clone)]
struct TrieNode {
    count: u32,
    is_leaf: bool,
    nei: Vec<Option<TrieNode>>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            count: 0,
            is_leaf: false,
            nei: vec![None; 10],
        }
    }
}

fn add_to_trie(node: &mut TrieNode, label_set: &Vec<u32>, label_set_idx: usize) {
    node.count += 1;

    if label_set_idx >= label_set.len() {
        node.is_leaf = true;
        return;
    }

    if node.nei.get(label_set[label_set_idx] as usize).is_none() {
        node.nei[label_set[label_set_idx] as usize] = Some(TrieNode::new());
    }

    let next_node = node
        .nei
        .get_mut(label_set[label_set_idx] as usize)
        .unwrap()
        .as_mut()
        .unwrap();

    add_to_trie(next_node, label_set, label_set_idx + 1);
}

fn build_trie(obj_n_label_set: Vec<(String, Vec<u32>)>) -> TrieNode {
    let mut root = TrieNode::new();

    for (_, label_set) in obj_n_label_set.iter() {
        add_to_trie(&mut root, label_set, 0);
    }

    root
}

fn materialize_hnsw() {
    todo!();
}

fn materialize_hnsw_for_trie(node: &mut TrieNode, maybe_paren_count: Option<u32>) {
    if maybe_paren_count != Some(node.count.ilog2()) {
        materialize_hnsw();
    } else {
        // inherit index
    }

    for nei in node.nei.iter_mut().flatten() {
        materialize_hnsw_for_trie(nei, Some(node.count));
    }
}

fn build_freq_map(obj_labels: &[Vec<String>]) -> HashMap<&String, u32> {
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
    let mut label_freq_vec: Vec<(&String, u32)> = Vec::with_capacity(label_freq_map.len());

    for (label, freq) in label_freq_map.iter() {
        label_freq_vec.push((label, *freq));
    }
    label_freq_vec.sort_by(|(_, fa), (_, fb)| fb.cmp(fa));

    label_freq_vec
}

fn generate_label_ids<'a>(label_freq_vec: &Vec<(&'a String, u32)>) -> HashMap<&'a String, u32> {
    let mut label_id_map: HashMap<&'a String, u32> = HashMap::new();
    for (next_id, (label, _)) in (0_u32..).zip(label_freq_vec.iter()) {
        label_id_map.insert(label, next_id);
    }

    label_id_map
}

fn build_obj_n_label_id_set(
    obj_labels: &[Vec<String>],
    label_id_map: &HashMap<&String, u32>,
) -> Vec<(String, Vec<u32>)> {
    let mut obj_n_label_set_vec: Vec<(String, Vec<u32>)> = Vec::new();
    for obj_n_labels_vec in obj_labels.iter() {
        let mut it = obj_n_labels_vec.iter();

        let Some(obj) = it.next() else {
            continue;
        };

        let mut label_set: Vec<u32> = Vec::new();
        for label_str in it {
            if let Some(&label_id) = label_id_map.get(label_str) {
                label_set.push(label_id);
            };
        }

        label_set.sort_unstable();

        obj_n_label_set_vec.push((obj.clone(), label_set));
    }

    obj_n_label_set_vec.sort_by(|(_, label_set_a), (_, label_set_b)| label_set_a.cmp(label_set_b));

    obj_n_label_set_vec
}

fn prepreocess_obj_n_labels(obj_labels: &[Vec<String>]) -> Vec<(String, Vec<u32>)> {
    let label_freq_map = build_freq_map(obj_labels);

    let label_freq_vec = sort_label_by_freq(&label_freq_map);

    let label_id_map = generate_label_ids(&label_freq_vec);

    build_obj_n_label_id_set(obj_labels, &label_id_map)
}

fn main() {
    let obj_labels: Vec<Vec<String>> = Vec::new();
    let obj_n_label_set = prepreocess_obj_n_labels(&obj_labels);

    let mut root = build_trie(obj_n_label_set);

    materialize_hnsw_for_trie(&mut root, None);
}
