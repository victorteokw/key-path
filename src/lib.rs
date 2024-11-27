use core::fmt::{Display, Formatter};
use std::ops::{Add, Index, Range};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item {
    Key(String),
    Index(usize),
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Item::Key(s) => f.write_str(s.as_ref()),
            Item::Index(n) => f.write_str(&n.to_string()),
        }
    }
}

impl Item {

    pub fn is_key(&self) -> bool {
        use Item::*;
        match self {
            Key(_) => true,
            Index(_) => false,
        }
    }

    pub fn is_index(&self) -> bool {
        use Item::*;
        match self {
            Key(_) => false,
            Index(_) => true,
        }
    }

    pub fn as_key(&self) -> Option<&str> {
        use Item::*;
        match self {
            Key(v) => Some(v.as_ref()),
            Index(_) => None,
        }
    }

    pub fn as_index(&self) -> Option<usize> {
        use Item::*;
        match self {
            Key(_) => None,
            Index(v) => Some(*v),
        }
    }
}

impl From<usize> for Item {
    fn from(index: usize) -> Self {
        use Item::*;
        Index(index)
    }
}

impl From<&str> for Item {
    fn from(key: &str) -> Self {
        use Item::*;
        Key(String::from(key))
    }
}

impl From<String> for Item {
    fn from(key: String) -> Self {
        use Item::*;
        Key(String::from(key))
    }
}

impl From<&String> for Item {
    fn from(key: &String) -> Self {
        use Item::*;
        Key(String::from(key.as_str()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyPath {
    items: Vec<Item>
}

impl KeyPath {

    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, index: usize) -> Option<&Item> {
        self.items.get(index)
    }

    pub fn last(&self) -> Option<&Item> {
        self.items.last()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> KeyPathIter {
        KeyPathIter { key_path: self, index: 0 }
    }
}

impl Default for KeyPath {
    fn default() -> Self {
        Self { items: vec![] }
    }
}

impl AsRef<KeyPath> for KeyPath {
    fn as_ref(&self) -> &KeyPath {
        &self
    }
}

impl Display for KeyPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let s = self.items.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(".");
        f.write_str(&s)
    }
}

impl From<KeyPath> for String {
    fn from(value: KeyPath) -> Self {
        value.to_string()
    }
}

impl From<&KeyPath> for String {
    fn from(value: &KeyPath) -> Self {
        value.to_string()
    }
}

impl<'a, T> Add<T> for &KeyPath where T: Into<Item> {
    type Output = KeyPath;

    fn add(self, rhs: T) -> Self::Output {
        let mut items = self.items.clone();
        items.push(rhs.into());
        KeyPath { items }
    }
}

impl<'a, T> Add<T> for KeyPath where T: Into<Item> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        (&self).add(rhs)
    }
}

impl Index<usize> for KeyPath {
    type Output = Item;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl Index<Range<usize>> for KeyPath {
    type Output = [Item];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.items[index]
    }
}

impl From<&[Item]> for KeyPath {
    fn from(items: &[Item]) -> Self {
        Self { items: items.to_vec() }
    }
}

pub struct KeyPathIter<'a> {
    key_path: &'a KeyPath,
    index: usize,
}

impl<'a> Iterator for KeyPathIter<'a> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.key_path.get(self.index);
        self.index += 1;
        result
    }
}

impl<'a> IntoIterator for &'a KeyPath {
    type Item = &'a Item;
    type IntoIter = KeyPathIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        KeyPathIter { key_path: self, index: 0 }
    }
}

impl<'a> IntoIterator for KeyPath {
    type Item = Item;
    type IntoIter = <Vec<Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

#[macro_export]
macro_rules! path {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(path!(@single $rest)),*]));
    (@item $other: expr) => ($crate::Item::from($other));
    ($($key:expr,)+) => { path!($($key),+) };
    ($($key:expr),*) => {
        {
            let _cap = path!(@count $($key),*);
            let mut _items = ::std::vec::Vec::with_capacity(_cap);
            $(
                let _ = _items.push(path!(@item $key));
            )*
            $crate::KeyPath::new(_items)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_works_for_empty() {
        let result = path![];
        assert_eq!(result, KeyPath::default());
    }

    #[test]
    fn macro_works_for_2_strings() {
        let result = path!["a", "b"];
        assert_eq!(result, KeyPath { items: vec![Item::Key("a".into()), Item::Key("b".into())]});
    }

    #[test]
    fn macro_works_for_2_numbers() {
        let result = path![2, 5];
        assert_eq!(result, KeyPath { items: vec![Item::Index(2), Item::Index(5)]});
    }

    #[test]
    fn macro_works_for_2_mixed_items() {
        let string = "where".to_owned();
        let result = path![string, 5];
        assert_eq!(result, KeyPath { items: vec![Item::Key("where".to_owned().into()), Item::Index(5)]});
    }

    #[test]
    fn macro_works_for_2_items_with_trailing_comma() {
        let string = "where".to_owned();
        let result = path![string, 5,];
        assert_eq!(result, KeyPath { items: vec![Item::Key("where".to_owned().into()), Item::Index(5)]});
    }

    #[test]
    fn macro_works_for_3_items() {
        let string = "where".to_owned();
        let result = path![string, 5, 7];
        assert_eq!(result, KeyPath { items: vec![Item::Key("where".to_owned().into()), Item::Index(5), Item::Index(7)]});
    }

    #[test]
    fn macro_works_for_3_items_with_trailing_comma() {
        let string = "where".to_owned();
        let result = path![string, 5, 7, ];
        assert_eq!(result, KeyPath { items: vec![Item::Key("where".to_owned().into()), Item::Index(5), Item::Index(7)]});
    }

    #[test]
    fn add_works_for_number() {
        let path = KeyPath::default();
        let result = path + 45;
        assert_eq!(result, KeyPath { items: vec![Item::Index(45)] })
    }

    #[test]
    fn add_works_for_str() {
        let path = KeyPath::default();
        let result = path + "";
        assert_eq!(result, KeyPath { items: vec![Item::Key("".into())] })
    }

    #[test]
    fn add_works_for_string() {
        let path = KeyPath::default();
        let result = path + "a".to_owned();
        assert_eq!(result, KeyPath { items: vec![Item::Key("a".to_owned().into())] })
    }

    #[test]
    fn add_works_for_string_ref() {
        let path = KeyPath::default();
        let string = "abc".to_owned();
        let string_ref = &string;
        let result = path + string_ref;
        assert_eq!(result, KeyPath { items: vec![Item::Key("abc".into())] })
    }

    #[test]
    fn key_path_can_be_debug_printed() {
        let path = path!["where", "items", 5, "name"];
        let result = format!("{}", path);
        assert_eq!(&result, "where.items.5.name");
    }

    #[test]
    fn index_works() {
        let path = path!["orderBy", "name"];
        let result = &path[0];
        assert_eq!(result, &("orderBy".into()))
    }

    #[test]
    fn index_with_range_works() {
        let path = path!["orderBy", "name", 3, "good"];
        let result = &path[0..2];
        assert_eq!(result, &[Item::Key("orderBy".to_string()), Item::Key("name".to_string())])
    }

    #[test]
    fn get_works() {
        let path = path!["orderBy", "name"];
        let result = path.get(0).unwrap();
        assert_eq!(result, &("orderBy".into()))
    }

    #[test]
    fn last_works() {
        let path = path!["orderBy", "name"];
        let result = path.last().unwrap();
        assert_eq!(result, &("name".into()))
    }

    #[test]
    fn as_ref_works() {
        let path = path!["a", "b"];
        let path2 = path.as_ref();
        let path3 = path2.as_ref();
        assert_eq!(&path, path2);
        assert_eq!(path2, path3);

    }

    #[test]
    fn to_string_works() {
        let path = path!["a", 2, "3"];
        assert_eq!(&path.to_string(), "a.2.3");
    }

    #[test]
    fn ref_to_string_works() {
        let path = path!["a", 2, "3"];
        let path_ref = &path;
        assert_eq!(&path_ref.to_string(), "a.2.3");
    }

    #[test]
    fn iter_works() {
        let path = path!["a", 2, "3"];
        let path = &path;
        let mut result = "".to_owned();
        for item in path {
            result += format!("{}", item).as_str();
        }
        assert_eq!(&result, "a23");
    }

    #[test]
    fn into_iter_works() {
        let path = path!["a", 2, "3"];
        let mut result = "".to_owned();
        for item in path {
            result += format!("{}", item).as_str();
        }
        assert_eq!(&result, "a23");
    }
}
