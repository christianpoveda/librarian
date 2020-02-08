# Librarian

This is a tiny command-line application to manage your digital library.

## Why?

I never found a satisfying way of storing and searching my digital library.
Using the documents' metadata sounded like a good idea, but then I realized that
editing the metadata of several file formats would require having more
than one application to do it. Then I wrote this small app to handle it instead.

## How?

- Librarian keeps an index with all your documents' metadata at
  `~/.library/index.json`.

- When you store a document using `librarian store`, the file is copied to the
  `~/.library` folder and the metadata is added to the index file. Each
  document is indexed using the hash of the file. Additionally you can provide
  the ISBN of the document if it has one and Librarian will try to recover its
  metadata from Open Library.

- Then you can search in your library using `librarian find`.

- Once you found the document, you can open it using `librarian open` with the
  document's hash. This is equivalent to using `open` or `xdg-open`.

For more help, run `librarian help`.

## Installation

Clone this repository, build it using `cargo build --release` and put the
binary in your `$PATH`.

## Stability

The API of Librarian will be susceptible to change until it reaches version
`1.0.0`. However, I won't be changing the format of the index unless it is in a
backwards-compatible way. So if you want to play with it and give it a try, you
can rest assured that the next update of Librarian won't mess up your index.

## Contributions

Please do! I'm more than happy to receive suggestions, questions, issues, PRs
and so.
