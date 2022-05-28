use epub::doc::EpubDoc;

fn main() {
    let doc = EpubDoc::new("Beyond the Basic Stuff with Python.epub");
    assert!(doc.is_ok());
    let doc = doc.unwrap();
}