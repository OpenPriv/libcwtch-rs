use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
/// Struct to serialize/deserialize events coming off the Cwtch appbus
pub struct CwtchEvent {
    pub event_type: String,
    pub event_ID: String, // event_ID because golang naming converntions in libCwtch-go
    pub data: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Struct to serialize/deserialize contacts coming from libcwtch-go
pub struct Contact {
    pub onion: String,
    pub name: String,
    pub status: String,
    pub authorization: String,
    pub is_group: bool,
    //attr: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
/// Struct to serialize/deserialize servers coming from libcwtch-go
pub struct Server {
    pub onion: String,
    pub status: String,
}

#[derive(Debug)]
/// Struct to serialize/deserialize profiles coming from libcwtch-go
pub struct Profile {
    pub onion: String,
    pub nick: String,
    pub image_path: String,
    pub attr: HashMap<String,String>,
    pub contacts: HashMap<String, Contact>,
    pub servers: HashMap<String, Server>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Struct to serialize/deserialize messages sent over Cwtch between profiles / contacts
pub struct Message {
    pub o: i64,
    pub d: String
}

impl Profile {
    pub fn new(identity: &str, name: &str, picture: &str, contacts_json: &str, server_list: &str) -> Profile {
        let contacts = Profile::process_contacts(contacts_json);
        let servers = Profile::process_servers(server_list);
        Profile{ onion: identity.to_string(), nick: name.to_string(), image_path: picture.to_string(), attr: Default::default(), contacts: contacts, servers: servers }
    }

    fn process_contacts(constacts_json: &str) -> HashMap<String, Contact> {
        let mut contacts: HashMap<String, Contact> = HashMap::new();
        if constacts_json == "null" {
            return contacts;
        }
        println!("contacts_json: '{}'", constacts_json);
        let contacts_map: Vec<Contact> = serde_json::from_str(constacts_json).unwrap();
        for contact in contacts_map {
            contacts.insert(contact.onion.clone(), contact);
        }
        contacts
    }

    fn process_servers(servers_json: &str) -> HashMap<String, Server> {
        let mut servers: HashMap<String, Server> = HashMap::new();
        if servers_json == "null" {
            return servers;
        }
        let servers_map: Vec<Server> = serde_json::from_str(servers_json).unwrap();
        for server in servers_map {
            servers.insert(server.onion.clone(), server);
        }
        servers
    }
}