use std::{fmt::Debug, str::FromStr};

pub trait Sorted {
    fn sorted_unstable(&self) -> Self;
}

impl<T> Sorted for Vec<T>
where
    T: Copy + Ord,
{
    fn sorted_unstable(&self) -> Self {
        let mut data = self.clone();
        data.sort_unstable();
        data
    }
}

pub struct Fancy<T>(pub T);

impl<T> Fancy<T> {
    pub fn reveal(self) -> T {
        self.0
    }
}

impl<T> FromIterator<T> for Fancy<(Vec<T>, Vec<T>)> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let mut xs = Vec::new();
        let mut ys = Vec::new();

        while let (Some(x), Some(y)) = (iter.next(), iter.next()) {
            xs.push(x);
            ys.push(y);
        }

        Fancy((xs, ys))
    }
}

pub trait SpaceSeperated {
    fn space_seperated<T: FromStr<Err = impl Debug>>(&self) -> impl Iterator<Item = T>;
}

impl SpaceSeperated for &str {
    fn space_seperated<T: FromStr<Err = impl Debug>>(&self) -> impl Iterator<Item = T> {
        self.split_ascii_whitespace()
            .map(|part| str::parse::<T>(part).unwrap())
    }
}

pub fn space_seperated_to_vec<T: FromStr<Err = impl Debug>>(input: &str) -> Vec<T> {
    input
        .split_ascii_whitespace()
        .map(|part| str::parse::<T>(part).unwrap())
        .collect()
}

pub struct TailsIterator<'a> {
    collection: &'a str,
    index: usize,
    end: usize,
}

impl<'a> Iterator for TailsIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.end {
            let tail = &self.collection[self.index..];
            self.index += 1;
            Some(tail)
        } else {
            None
        }
    }
}

pub fn tails(collection: &str) -> TailsIterator {
    TailsIterator {
        collection,
        index: 0,
        end: collection.len(),
    }
}
