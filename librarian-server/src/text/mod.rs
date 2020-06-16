use librarian_core::Doc;

use crate::library::DocId;

mod index;

use index::TextIndex;

#[derive(Default)]
pub(crate) struct SearchEngine {
    title: TextIndex<3>,
    authors: TextIndex<3>,
    keywords: TextIndex<3>,
}

impl SearchEngine {
    pub(crate) fn index(&mut self, id: DocId, doc: &Doc) -> DocId {
        self.title.insert(id, doc.title.to_lowercase().as_bytes());

        self.authors.insert_many(
            id,
            doc.authors
                .iter()
                .map(|text| text.to_lowercase().into_bytes()),
        );

        self.keywords.insert_many(
            id,
            doc.keywords
                .iter()
                .map(|text| text.to_lowercase().into_bytes()),
        );

        id
    }

    pub(crate) fn deindex(&mut self, id: &DocId) {
        self.title.remove(id);
        self.authors.remove(id);
        self.keywords.remove(id);
    }

    pub(crate) fn search(&self, text: &str, limit: usize) -> Vec<(DocId, f32)> {
        let text = text.to_lowercase();
        let text = text.as_bytes();

        let mut scores = self.title.search(text);

        for (id, score) in self.authors.search(text) {
            *scores.entry(id).or_insert(0.0) += score;
        }

        for (id, score) in self.keywords.search(text) {
            *scores.entry(id).or_insert(0.0) += score;
        }

        let mut scores: Vec<_> = scores.into_iter().collect();
        scores.sort_by(|(_, s1), (_, s2)| s2.partial_cmp(s1).unwrap());
        scores.truncate(limit);
        scores
    }
}
