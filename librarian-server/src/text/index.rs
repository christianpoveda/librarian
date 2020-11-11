use std::collections::{BTreeMap, HashMap};
use std::convert::TryFrom;

use crate::library::DocId;

#[derive(Default)]
pub(crate) struct TextIndex<const N: usize> {
    grams: HashMap<[u8; N], Freqs>,
    total_docs: f32,
}

#[derive(Default)]
struct Freqs {
    freqs: BTreeMap<DocId, f32>,
    max_freq: f32,
}

impl Freqs {
    fn ids(&self) -> impl Iterator<Item = &DocId> {
        self.freqs.keys()
    }

    fn docs(&self) -> usize {
        self.freqs.len()
    }

    fn freq(&self, id: &DocId) -> f32 {
        let freq = self.freqs.get(id).copied().unwrap_or(0.0);
        0.5 + 0.5 * freq / self.max_freq as f32
    }

    fn increase(&mut self, id: DocId) {
        let freq = self.freqs.entry(id).or_insert(0.0);
        *freq += 1.0;
        if *freq > self.max_freq {
            self.max_freq = *freq;
        }
    }

    fn decrease(&mut self, id: &DocId) {
        if let Some(freq) = self.freqs.get_mut(id) {
            if *freq == 1.0 {
                self.freqs.remove(id);
            } else {
                *freq -= 1.0;
            }
        }
    }
}

impl<const N: usize> TextIndex<N> {
    pub(crate) fn search(&self, text: &[u8]) -> BTreeMap<DocId, f32> {
        let mut scores = BTreeMap::new();

        for gram in text.windows(N) {
            if let Some(freqs) = self.grams.get(gram) {
                for id in freqs.ids() {
                    let freq = freqs.freq(id);
                    let inv = ((self.total_docs as f32) / (freqs.docs().max(1) as f32)).ln();
                    let score = scores.entry(id.clone()).or_insert(0.0);
                    *score += freq * inv;
                }
            }
        }

        scores
    }

    pub(crate) fn insert(&mut self, id: DocId, text: &[u8]) {
        for gram in text.windows(N) {
            let gram = <[u8; N]>::try_from(gram).unwrap();
            self.grams
                .entry(gram)
                .or_insert_with(|| Freqs::default())
                .increase(id.clone());
        }
        self.total_docs += 1.0;
    }

    pub(crate) fn insert_many(&mut self, id: DocId, texts: impl Iterator<Item = Vec<u8>>) {
        for text in texts {
            for gram in text.windows(N) {
                let gram = <[u8; N]>::try_from(gram).unwrap();
                self.grams
                    .entry(gram)
                    .or_insert_with(|| Freqs::default())
                    .increase(id.clone());
            }
        }
        self.total_docs += 1.0;
    }

    pub(crate) fn remove(&mut self, id: &DocId) {
        for freqs in self.grams.values_mut() {
            freqs.decrease(&id);
        }
        self.total_docs -= 1.0;
    }
}
