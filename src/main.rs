use std::collections::HashMap;

struct Arena {
    objs: Vec<String>,
    labels: Vec<String>,
    label_freq: Vec<u32>,
    label_id_map: HashMap<String, usize>,
    obj_n_label_set_vec: Vec<(usize, Vec<usize>)>,
    trie_nodes: Vec<TrieNode>,
}

impl Arena {
    fn new() -> Self {
        let objs: Vec<String> = Vec::with_capacity(1024);
        let labels: Vec<String> = Vec::with_capacity(1024);
        let label_freq: Vec<u32> = Vec::with_capacity(1024);
        let label_id_map: HashMap<String, usize> = HashMap::with_capacity(1024);
        let obj_n_label_set_vec: Vec<(usize, Vec<usize>)> = Vec::with_capacity(1024);
        let trie_nodes: Vec<TrieNode> = Vec::with_capacity(1024);
        Arena {
            objs,
            labels,
            label_freq,
            label_id_map,
            obj_n_label_set_vec,
            trie_nodes,
        }
    }
}

struct TrieNode {
    count: u32,
    nei: Vec<Option<usize>>,
    objs: Vec<usize>,
    paren: Option<usize>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            count: 0,
            nei: vec![None; 1024],
            objs: Vec::new(),
            paren: None,
        }
    }
}

struct Trie {}

impl Trie {
    fn add_to_trie(
        trie_nodes: &mut Vec<TrieNode>,
        obj_id: usize,
        label_set: &[usize],
        terminal_label: usize,
    ) {
        let mut curr_idx = 0;
        trie_nodes[curr_idx].count += 1;

        for &label_id in label_set.iter().chain(std::iter::once(&terminal_label)) {
            let next_idx = match trie_nodes[curr_idx].nei.get(label_id) {
                Some(&Some(idx)) => idx,
                Some(&None) => {
                    let idx = trie_nodes.len();
                    trie_nodes.push(TrieNode::new());
                    trie_nodes[curr_idx].nei[label_id] = Some(idx);
                    idx
                }
                None => return,
            };
            trie_nodes[next_idx].count += 1;
            trie_nodes[next_idx].paren = Some(curr_idx);
            curr_idx = next_idx;
        }

        trie_nodes[curr_idx].objs.push(obj_id);
    }

    fn build_trie(arena: &mut Arena) {
        arena.trie_nodes.push(TrieNode::new());
        let terminal_label = arena.labels.len();

        for (obj_id, label_set) in &arena.obj_n_label_set_vec {
            Self::add_to_trie(&mut arena.trie_nodes, *obj_id, label_set, terminal_label);
        }
    }

    fn materialize_hnsw_for_trie(
        trie_nodes: &Vec<TrieNode>,
        node_idx: usize,
        maybe_parent_log_count: Option<u32>,
    ) {
        let Some(log_size) = trie_nodes[node_idx].count.checked_ilog2() else {
            return;
        };

        if maybe_parent_log_count != Some(log_size) {
            HNSWGraph::materialize_hnsw();
        } else {
            // inherit index
        }

        for &neigh in trie_nodes[node_idx].nei.iter().flatten() {
            Self::materialize_hnsw_for_trie(trie_nodes, neigh, Some(log_size));
        }
    }
}

struct ObjectNLabels {}

impl ObjectNLabels {
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
        let mut entries: Vec<(&String, u32)> =
            label_freq_map.iter().map(|(&l, &f)| (l, f)).collect();
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
            arena.objs.push(obj.clone());

            let mut label_set: Vec<usize> = Vec::new();
            for label_str in it {
                if let Some(&label_id) = arena.label_id_map.get(label_str) {
                    label_set.push(label_id);
                };
            }
            label_set.sort_unstable();

            arena
                .obj_n_label_set_vec
                .push((arena.objs.len() - 1, label_set));
        }

        arena
            .obj_n_label_set_vec
            .sort_by(|(_, label_set_a), (_, label_set_b)| label_set_a.cmp(label_set_b));
    }

    fn prepreocess_obj_n_labels(arena: &mut Arena, obj_labels: &[Vec<String>]) {
        let label_freq_map = Self::build_label_freq_map(obj_labels);

        Self::add_label_freq_to_arena_sorted(arena, &label_freq_map);

        Self::build_obj_n_label_id_set(arena, obj_labels);
    }
}

struct HNSWGraph {}

impl HNSWGraph {
    fn materialize_hnsw() {
        todo!();
    }
}

fn main() {
    let mut arena = Arena::new();
    let obj_labels: Vec<Vec<String>> = Vec::new();
    ObjectNLabels::prepreocess_obj_n_labels(&mut arena, &obj_labels);

    Trie::build_trie(&mut arena);

    if !arena.trie_nodes.is_empty() {
        Trie::materialize_hnsw_for_trie(&arena.trie_nodes, 0, None);
    }
}
