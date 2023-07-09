use std::fs;
use std::path::{Path, PathBuf};
use yrs::{Doc, GetString, ReadTxn, StateVector, Text, TextRef, Transact, Update};
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;

fn main() {
    let server = MdDocument::new("./bin/server.bin", "test.md");

    let client_a = MdDocument::new("./bin/client-a.bin", "test.md");
    // client_a.insert(0, "Hello");
    // client_a.insert(5, " Bob");

    let client_b = MdDocument::new("./bin/client-b.bin", "test.md");
    // client_b.insert(0, "Hello");
    // client_b.insert(5, " Alice");

    client_a.sync_both_ways(&server);
    client_b.sync_both_ways(&server);

    dbg!(client_a.get_content());
    dbg!(client_b.get_content());
    dbg!(server.get_content());
}

struct MdDocument {
    fs_path: PathBuf,
    doc: Doc,
    text: TextRef,
}

impl MdDocument {
    pub fn new(
        fs_path: impl Into<PathBuf>,
        document_name: impl Into<String>,
    ) -> Self {
        Self {
            fs_path,
            doc,
            text,
        }
    }

    pub fn insert(
        &self,
        index: u32,
        content: impl Into<String>,
    ) {
        let mut transaction = self.doc.transact_mut();
        self.text.insert(&mut transaction, index, &*content.into());
    }

    pub fn get_content(&self) -> String {
        self.text.get_string(&self.doc.transact())
    }

    pub fn sync_both_ways(&self, remote: &MdDocument) {
        self.sync_with(remote);
        remote.sync_with(self);
    }

    fn save_changes(&self) {
        fs::create_dir_all(&self.fs_path.parent().unwrap())
            .unwrap();

        fs::write(&self.fs_path, &self.doc
            .transact()
            .store()
            .encode_v1()
        )
            .unwrap();
    }

    pub fn sync_with(&self, remote: &MdDocument) {
        // Messages that could be send using websockets. Syncing should be bi-directional, eg both
        // the server and client can initiate syncing.

        // Start sync ->
        // Start message should send the path of the document so it can be loaded into memory or
        // created on the remote.

        // <- Receive remote timestamp
        let remote_timestamp = remote.doc
            .transact()
            .state_vector()
            .encode_v1();

        // Send local updates ->
        let update = self.doc
            .transact()
            .encode_diff_v1(
                &StateVector::decode_v1(&remote_timestamp)
                    .unwrap()
            );

        // On remote <>
        remote.doc
            .transact_mut()
            .apply_update(Update::decode_v1(&update).unwrap());

        remote.save_changes();
    }
}
