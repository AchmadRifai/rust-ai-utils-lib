use std::{collections::HashMap, hash::Hash};

pub trait ToVec: Iterator {
    fn to_vec(self) -> Vec<Self::Item>;
}

impl<I> ToVec for I
where
    I: Iterator,
{
    fn to_vec(self) -> Vec<Self::Item> {
        let mut result = vec![];
        for i in self {
            result.push(i);
        }
        result
    }
}

pub trait Distinct: Iterator {
    fn distinct(self) -> Vec<Self::Item>;
}

impl<I: Iterator> Distinct for I
where
    I::Item: PartialEq,
{
    fn distinct(self) -> Vec<Self::Item> {
        let mut tmp: Vec<Self::Item> = vec![];
        for i in self {
            if !tmp.contains(&i) {
                tmp.push(i);
            }
        }
        tmp
    }
}

pub trait GroupBy: Iterator {
    fn group_by<T: Hash + Eq, U: Fn(&Self::Item) -> T>(
        self,
        get_key: U,
    ) -> HashMap<T, Vec<Self::Item>>;
}

impl<I: Iterator> GroupBy for I {
    fn group_by<T: Hash + Eq, U: Fn(&Self::Item) -> T>(
        self,
        get_key: U,
    ) -> HashMap<T, Vec<Self::Item>> {
        let mut dict = HashMap::new();
        for v in self {
            let k = get_key(&v);
            match dict.get_mut(&k) {
                None => {
                    let mut list = vec![];
                    list.push(v);
                    dict.insert(k, list);
                },
                Some(list) => list.push(v)
            }
        }
        dict
    }
}

#[cfg(test)]
mod ranges_tests {
    use super::*;

    #[test]
    fn to_vec() {
        let jarak = (0..100).filter(|i| i % 2 == 0).to_vec();
        println!("Result : {:?}", jarak);
    }

    #[test]
    fn distinct() {
        let result = (0..100).map(|x| x % 3).distinct();
        println!("Result : {:?}", result);
    }

    #[derive(Debug)]
    struct Grouped {
        cat: &'static str,
        val: i32,
    }

    impl std::fmt::Display for Grouped {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Grouped[cat={}, val={}]", self.cat, self.val)
        }
    }

    #[test]
    fn group_by() {
        let result = (0..10)
            .map(|i| Grouped {
                cat: if i % 2 == 0 { "Genap" } else { "Ganjil" },
                val: i,
            })
            .group_by(|g| g.cat);
        println!("Result : {:?}", result)
    }
}
