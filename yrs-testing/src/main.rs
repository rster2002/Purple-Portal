use std::fs;
use std::path::PathBuf;

fn main() {
    let mut server = MdDocument::new(1, "./bin/server.bin", "test.md");

    let mut client_a = MdDocument::new(2, "./bin/client-a.bin", "test.md");
    client_a.insert(0, "Hello");
    client_a.insert(5, " Bob");

    let mut client_b = MdDocument::new(3, "./bin/client-b.bin", "test.md");
    client_b.insert(0, "Hello");
    client_b.insert(5, " Alice");

    client_a.sync_both_ways(&mut server);
    client_b.sync_both_ways(&mut server);

    dbg!(client_a.get_content());
    dbg!(client_b.get_content());
    dbg!(server.get_content());
}

struct MdDocument {
    fs_path: PathBuf,
    text: ditto::Text,
}

impl MdDocument {
    pub fn new(
        id: u32,
        fs_path: impl Into<PathBuf>,
        document_name: impl Into<String>,
    ) -> Self {
        let fs_path = fs_path.into();

        let mut i = ditto::Text::new();
        let _ = i.add_site_id(id);

        Self {
            fs_path,
            text: i,
        }
    }

    pub fn insert(
        &mut self,
        index: u32,
        content: impl Into<String>,
    ) {
        self.text.replace(index as usize, 0, &*content.into());
    }

    pub fn remove(
        &mut self,
        index: u32,
        len: u32,
    ) {
        self.text.replace(index as usize, len as usize, "");
    }

    pub fn get_content(&self) -> String {
        self.text.local_value()
    }

    pub fn sync_both_ways(&mut self, remote: &mut MdDocument) {
        self.sync_with(remote);
        remote.sync_with(self);
    }

    fn save_changes(&self) {
        // fs::create_dir_all(&self.fs_path.parent().unwrap())
        //     .unwrap();
        //
        // fs::write(&self.fs_path, &self.doc
        //     .transact()
        //     .state_vector()
        //     .encode_v1()
        // )
        //     .unwrap();
    }

    pub fn sync_with(&mut self, remote: &mut MdDocument) {
        remote.text.merge(self.text.state())
            .unwrap();

        // // Messages that could be send using websockets. Syncing should be bi-directional, eg both
        // // the server and client can initiate syncing.
        //
        // // Start sync ->
        // // Start message should send the path of the document so it can be loaded into memory or
        // // created on the remote.
        //
        // // <- Receive remote timestamp
        // let remote_timestamp = remote.doc
        //     .transact()
        //     .state_vector()
        //     .encode_v1();
        //
        // // Send local updates ->
        // let update = self.doc
        //     .transact()
        //     .encode_diff_v1(
        //         &StateVector::decode_v1(&remote_timestamp)
        //             .unwrap()
        //     );
        //
        // // On remote <>
        // remote.doc
        //     .transact_mut()
        //     .apply_update(Update::decode_v1(&update).unwrap());
        //
        // remote.save_changes();
    }
}
