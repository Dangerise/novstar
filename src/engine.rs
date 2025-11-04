 use super::*;
 
 use std::collections::HashMap;
 
 #[derive(Debug, Clone, PartialEq)]
 pub enum SearchResult {
     Name(String),
     Id(usize),
 }
 
 #[derive(Debug, Clone)]
 pub struct Engine<'a> {
     pub data: &'a Data,
     pub map: HashMap<&'a str, Vec<usize>>,
     pub results: Vec<SearchResult>,
 }
 
 impl<'a> Engine<'a> {
     pub fn from_data(data: &'a Data) -> Self {
         let mut map: HashMap<_, Vec<usize>> = HashMap::new();
         for (idx, Comment { book_name, .. }) in data.comments.iter().enumerate() {
             map.entry(book_name.as_str()).or_default().push(idx);
         }
         Self {
             data,
             map,
             results: Default::default(),
         }
     }
     pub fn get_book(&self, book_name: &str) -> Option<impl Iterator<Item = &str> + Clone> {
         let iter = self
             .map
             .get(book_name)?
             .iter()
             .map(|&x| self.data.comments[x].content.as_str());
         Some(iter)
     }
     pub fn search(&mut self, pattern: &[&str]) -> eyre::Result<()> {
         let Self { data, map, results } = self;
 
         results.clear();
         for (&book_name, list) in map.iter() {
             if book_name == "Other" {
                 continue;
             }
             if pattern.iter().all(|&pat| {
                 list.iter()
                     .map(|&x| &data.comments[x].content)
                     .any(|c| c.contains(pat))
             }) {
                 results.push(SearchResult::Name(book_name.to_string()));
             }
         }
 
         let list = map
             .get("Other")
             .ok_or_else(|| eyre::eyre!("Other nou found"))?
             .iter()
             .map(|&x| (x, data.comments[x].content.as_str()));
         for (id, content) in list {
             if pattern.iter().all(|&pat| content.contains(pat)) {
                 results.push(SearchResult::Id(id));
             }
         }
         Ok(())
     }
 }