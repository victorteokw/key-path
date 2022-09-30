use core::fmt::{Display, Formatter, Write};
use std::ops::Add;
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item<'a> {
    Key(Cow<'a, str>),
    Index(usize),
}

impl Display for Item<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Item::Key(s) => f.write_str(s.as_ref()),
            Item::Index(n) => f.write_str(&n.to_string()),
        }
    }
}

impl Item<'_> {

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

impl From<usize> for Item<'_> {
    fn from(index: usize) -> Self {
        use Item::*;
        Index(index)
    }
}

impl<'a> From<&'a str> for Item<'a> {
    fn from(key: &'a str) -> Self {
        use Item::*;
        Key(Cow::from(key))
    }
}

impl From<String> for Item<'_> {
    fn from(key: String) -> Self {
        use Item::*;
        Key(Cow::from(key))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyPath<'a> {
    items: Vec<Item<'a>>
}

impl KeyPath<'_> {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl Display for KeyPath<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let s = self.items.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(".");
        f.write_str(&s)
    }
}

impl<'a, T> Add<T> for &KeyPath<'a> where T: Into<Item<'a>> {
    type Output = KeyPath<'a>;

    fn add(self, rhs: T) -> Self::Output {
        let mut items = self.items.clone();
        items.push(rhs.into());
        KeyPath { items }
    }
}

impl<'a, T> Add<T> for KeyPath<'a> where T: Into<Item<'a>> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        (&self).add(rhs)
    }
}

#[macro_export]
macro_rules! path {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(path!(@single $rest)),*]));
    (@item $other: expr) => (Item::from($other));
    ($($key:expr,)+) => { path!($($key),+) };
    ($($key:expr),*) => {
        {
            let _cap = path!(@count $($key),*);
            let mut _items = ::std::vec::Vec::with_capacity(_cap);
            $(
                let _ = _items.push(path!(@item $key));
            )*
            $crate::KeyPath { items: _items }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_works_for_empty() {
        let result = path![];
        assert_eq!(result, KeyPath::new());
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
        let path = KeyPath::new();
        let result = path + 45;
        assert_eq!(result, KeyPath { items: vec![Item::Index(45)] })
    }

    #[test]
    fn add_works_for_str() {
        let path = KeyPath::new();
        let result = path + "";
        assert_eq!(result, KeyPath { items: vec![Item::Key("".into())] })
    }

    #[test]
    fn add_works_for_string() {
        let path = KeyPath::new();
        let result = path + "a".to_owned();
        assert_eq!(result, KeyPath { items: vec![Item::Key("a".to_owned().into())] })
    }

    #[test]
    fn key_path_can_be_debug_printed() {
        let path = path!["where", "items", 5, "name"];
        let result = format!("{}", path);
        assert_eq!(&result, "where.items.5.name");
    }
}
