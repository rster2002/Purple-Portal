use diamond_types::list::encoding::EncodeOptions;
use diamond_types::list::{Branch, OpLog};
use diamond_types::AgentId;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut server = ServerFile::new("test");

    let mut client_a = ClientFile::new("alice", "test");
    client_a.use_remote_content(&server.get_op_log()); // Start connection, get ops from server

    client_a.insert(0, "Hello");
    client_a.insert(5, " Bob");

    server.use_remote_content(&client_a.get_op_log()); // Send ops to server
    client_a.use_remote_content(&server.get_op_log()); // Broadcast ops to clients

    let mut client_b = ClientFile::new("bob", "test");
    client_b.use_remote_content(&server.get_op_log()); // Start connection, get ops from server

    client_b.insert(0, "Hello");
    client_b.insert(5, " Alice");

    server.use_remote_content(&client_b.get_op_log()); // Send ops to server
    client_a.use_remote_content(&server.get_op_log()); // Broadcast ops to clients
    client_b.use_remote_content(&server.get_op_log()); // Broadcast ops to clients

    let mut client_c = ClientFile::new("celine", "test");
    client_c.use_remote_content(&server.get_op_log()); // Start connection, get ops from server

    dbg!(client_a.get_content());
    dbg!(client_b.get_content());
    dbg!(client_c.get_content());
}

struct ServerFile {
    name: String,
    op_log: OpLog,
}

impl ServerFile {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            op_log: OpLog::new(),
        }
    }

    pub fn get_op_log(&self) -> Vec<u8> {
        self.op_log.encode(EncodeOptions::default())
    }

    pub fn use_remote_content(&mut self, content: &Vec<u8>) {
        self.op_log
            .decode_and_add(content)
            .expect("TODO: panic message");
    }
}

struct ClientFile {
    agent: AgentId,
    name: String,
    op_log: OpLog,
}

impl ClientFile {
    pub fn new(agent_id: &str, name: impl Into<String>) -> Self {
        let mut op_log = OpLog::new();
        let agent = op_log.get_or_create_agent_id(agent_id);

        Self {
            agent,
            name: name.into(),
            op_log,
        }
    }

    pub fn insert(&mut self, index: usize, content: impl Into<String>) {
        self.op_log.add_insert(self.agent, index, &*content.into());
    }

    pub fn get_op_log(&self) -> Vec<u8> {
        self.op_log.encode(EncodeOptions::default())
    }

    pub fn use_remote_content(&mut self, content: &Vec<u8>) {
        self.op_log
            .decode_and_add(content)
            .expect("TODO: panic message");
    }

    pub fn get_content(&self) -> String {
        Branch::new_at_tip(&self.op_log).content().to_string()
    }
}
