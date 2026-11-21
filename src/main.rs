use std::collections::HashMap;

struct Arena {
    objs: Vec<String>,
    labels: Vec<String>,
    label_freq: Vec<u32>,
    label_id_map: HashMap<String, usize>,
    obj_n_label_set_vec: Vec<(String, Vec<usize>)>,
}

impl Arena {
    fn new() -> Self {
        let objs: Vec<String> = Vec::with_capacity(1024);
        let labels: Vec<String> = Vec::with_capacity(1024);
        let label_freq: Vec<u32> = Vec::with_capacity(1024);
        let label_id_map: HashMap<String, usize> = HashMap::with_capacity(1024);
        let obj_n_label_set_vec: Vec<(String, Vec<usize>)> = Vec::with_capacity(1024);
        Arena {
            objs,
            labels,
            label_freq,
            label_id_map,
            obj_n_label_set_vec,
        }
    }
}

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

fn add_to_trie(node: &mut TrieNode, label_set: &Vec<usize>, label_set_idx: usize) {
    node.count += 1;

    if label_set_idx >= label_set.len() {
        node.is_leaf = true;
        return;
    }

    if node.nei.get(label_set[label_set_idx]).is_none() {
        return;
    }

    if node.nei[label_set[label_set_idx]].is_none() {
        node.nei[label_set[label_set_idx]] = Some(TrieNode::new());
    }

    let next_node = node
        .nei
        .get_mut(label_set[label_set_idx])
        .unwrap()
        .as_mut()
        .unwrap();

    add_to_trie(next_node, label_set, label_set_idx + 1);
}

fn build_trie(arena: &Arena) -> TrieNode {
    let mut root = TrieNode::new();

    for (_, label_set) in arena.obj_n_label_set_vec.iter() {
        add_to_trie(&mut root, label_set, 0);
    }

    root
}

fn materialize_hnsw() {
    todo!();
}

fn materialize_hnsw_for_trie(node: &mut TrieNode, maybe_parent_log_count: Option<u32>) {
    let Some(log_size) = node.count.checked_ilog2() else {
        return;
    };

    if maybe_parent_log_count != Some(log_size) {
        materialize_hnsw();
    } else {
        // inherit index
    }

    for nei in node.nei.iter_mut().flatten() {
        materialize_hnsw_for_trie(nei, Some(log_size));
    }
}

fn build_label_freq_map(obj_labels: &[Vec<String>]) -> HashMap<&String, u32> {
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

fn add_label_freq_to_arena_sorted(arena: &mut Arena, label_freq_map: &HashMap<&String, u32>) {
    let mut entries: Vec<(&String, u32)> = label_freq_map.iter().map(|(&l, &f)| (l, f)).collect();
    entries.sort_by(|(_, fa), (_, fb)| fb.cmp(fa));

    for (idx, (label, freq)) in entries.iter().enumerate() {
        arena.labels.push((*label).clone());
        arena.label_freq.push(*freq);
        arena.label_id_map.insert((*label).clone(), idx);
    }
}

fn build_obj_n_label_id_set(arena: &mut Arena, obj_labels: &[Vec<String>]) {
    for obj_n_labels_vec in obj_labels.iter() {
        let mut it = obj_n_labels_vec.iter();

        let Some(obj) = it.next() else {
            continue;
        };

        let mut label_set: Vec<usize> = Vec::new();
        for label_str in it {
            if let Some(&label_id) = arena.label_id_map.get(label_str) {
                label_set.push(label_id);
            };
        }

        label_set.sort_unstable();

        arena.obj_n_label_set_vec.push((obj.clone(), label_set));
    }

    arena
        .obj_n_label_set_vec
        .sort_by(|(_, label_set_a), (_, label_set_b)| label_set_a.cmp(label_set_b));
}

fn populate_objs_in_arena(arena: &mut Arena, obj_labels: &[Vec<String>]) {
    for items in obj_labels.iter() {
        if let Some(obj) = items.first() {
            arena.objs.push(obj.clone());
        }
    }
}

fn prepreocess_obj_n_labels(arena: &mut Arena, obj_labels: &[Vec<String>]) {
    populate_objs_in_arena(arena, obj_labels);

    let label_freq_map = build_label_freq_map(obj_labels);

    add_label_freq_to_arena_sorted(arena, &label_freq_map);

    build_obj_n_label_id_set(arena, obj_labels);
}

fn main() {
    let mut arena = Arena::new();
    let obj_labels: Vec<Vec<String>> = Vec::new();
    prepreocess_obj_n_labels(&mut arena, &obj_labels);

    let mut root = build_trie(&arena);

    materialize_hnsw_for_trie(&mut root, None);
}
