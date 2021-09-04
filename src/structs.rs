use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CwtchEvent {
    pub event_type: String,
    pub event_ID: String,
    pub data: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    pub onion: String,
    pub name: String,
    pub status: String,
    pub authorization: String,
    pub isGroup: bool,
    //attr: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub onion: String,
    pub status: String,
}

#[derive(Debug)]
pub struct Profile {
    pub onion: String,
    pub nick: String,
    pub imagePath: String,
    pub attr: HashMap<String,String>,
    pub contacts: HashMap<String, Contact>,
    pub servers: HashMap<String, Server>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub o: i64,
    pub d: String
}

impl Profile {
    pub fn new(identity: &str, name: &str, picture: &str, contacts_json: &str, server_list: &str) -> Profile {
        let contacts = Profile::process_contacts(contacts_json);
        let servers = Profile::process_servers(server_list);
        Profile{ onion: identity.to_string(), nick: name.to_string(), imagePath: picture.to_string(), attr: Default::default(), contacts: contacts, servers: servers }
    }

    pub fn process_contacts(constacts_json: &str) -> HashMap<String, Contact> {
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

    pub fn process_servers(servers_json: &str) -> HashMap<String, Server> {
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