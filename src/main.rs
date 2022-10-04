use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct TrieNode<T> {
    key_char_: char,
    value_: Option<T>,
    children_: HashMap<char, TrieNode<T>>,
}

impl<T> TrieNode<T> {
    fn new(key_char: char, value: Option<T>) -> TrieNode<T> {
        TrieNode {
            value_: value,
            key_char_: key_char,
            children_: HashMap::new(),
        }
    }

    fn has_child(&self, key_char: char) -> bool {
        self.children_.contains_key(&key_char)
    }

    fn has_children(&self) -> bool {
        !self.children_.is_empty()
    }

    fn get_key_char(&self) -> char {
        self.key_char_
    }

    fn insert_child_node(
        &mut self,
        key_char: char,
        child: TrieNode<T>,
    ) -> Option<&mut TrieNode<T>> {
        if self.has_child(key_char) {
            return None;
        } else if key_char != child.get_key_char() {
            return None;
        } else {
            return match self.children_.insert(key_char, child) {
                Some(_) => None,
                None => Some(self.children_.get_mut(&key_char).unwrap()),
            };
        }
    }

    fn get_child_node(&mut self, key_char: char) -> Option<&mut TrieNode<T>> {
        self.children_.get_mut(&key_char)
    }

    fn remove_child_node(&mut self, key_char: char) -> Option<TrieNode<T>> {
        self.children_.remove(&key_char)
    }

    fn set_key_char(&mut self, key_char: char) {
        self.key_char_ = key_char;
    }

    fn get_children(&self) -> &HashMap<char, TrieNode<T>> {
        &self.children_
    }

    fn get_value(&self) -> Option<&T> {
        self.value_.as_ref()
    }

    fn set_value(&mut self, value: T) {
        self.value_ = Some(value);
    }
}

#[derive(Debug, PartialEq)]
struct Trie<T> {
    root_: TrieNode<T>,
}

impl<T> Trie<T> {
    fn new() -> Trie<T> {
        Trie {
            root_: TrieNode::new('\0', None),
        }
    }

    // Insert a key into the trie
    fn insert(&mut self, key: &str, value: T) -> bool {
        if key.is_empty() {
            return false;
        }

        let mut current_node = &mut self.root_;
        let chars_count = key.chars().count();
        for (i, c) in key.chars().enumerate() {
            if i == chars_count - 1 {
                break;
            }

            if !current_node.has_child(c) {
                current_node = current_node
                    .insert_child_node(c, TrieNode::new(c, None))
                    .unwrap();
            } else {
                current_node = current_node.get_child_node(c).unwrap();
            }
        }

        let last_char = key.chars().last().unwrap();
        if current_node.has_child(last_char) {
            current_node = current_node.get_child_node(last_char).unwrap();
            match current_node.get_value() {
                Some(_) => {
                    return false;
                }
                None => current_node.set_value(value),
            };
        } else {
            current_node = current_node
                .insert_child_node(last_char, TrieNode::new(last_char, Some(value)))
                .unwrap();
        }

        true
    }

    // Get Key Value
    fn get_value(&mut self, key: &str) -> Option<&T> {
        if key.is_empty() {
            return None;
        }

        let mut current_node = &mut self.root_;
        for c in key.chars() {
            if !current_node.has_child(c) {
                return None;
            } else {
                current_node = current_node.get_child_node(c).unwrap();
            }
        }

        current_node.get_value()
    }
}

fn main() {
    // TrieNode Insert Test
    let mut root = TrieNode::<u32>::new('a', None);
    let mut child = TrieNode::<u32>::new('b', None);
    let mut res = root.insert_child_node('b', child);

    // Get Key Char Test
    assert_ne!(res, None);
    assert_eq!(res.unwrap().get_key_char(), 'b');

    // Duplicate Key Insert
    child = TrieNode::new('b', None);
    res = root.insert_child_node('b', child);
    assert_eq!(res, None);

    // Mismismatch Key Insert
    child = TrieNode::new('b', None);
    res = root.insert_child_node('d', child);
    assert_eq!(res, None);

    // Get Key Char
    child = TrieNode::new('c', None);
    res = root.insert_child_node('c', child);
    assert_ne!(res, None);
    assert_eq!(res.unwrap().get_key_char(), 'c');

    // TrieNode Remove Test
    root.remove_child_node('b');
    assert_eq!(root.has_child('b'), false);
    assert_eq!(root.has_children(), true);
    assert_eq!(root.get_child_node('b'), None);

    root.remove_child_node('c');
    assert_eq!(root.has_child('c'), false);
    assert_eq!(root.has_children(), false);
    assert_eq!(root.get_child_node('c'), None);

    // Trie Test
    let mut trie = Trie::<&str>::new();

    // Trie Random Order Insert Test
    trie.insert("a", "one");
    trie.insert("aaa", "three");
    trie.insert("aaaa", "four");
    trie.insert("aa", "two");

    assert_eq!(trie.get_value("a"), Some(&"one"));
    assert_eq!(trie.get_value("aaa"), Some(&"three"));
    assert_eq!(trie.get_value("aaaa"), Some(&"four"));
    assert_eq!(trie.get_value("aa"), Some(&"two"));

    // Trie Insert Duplicate Key Test
    assert_eq!(trie.insert("a", "one"), false);
}
