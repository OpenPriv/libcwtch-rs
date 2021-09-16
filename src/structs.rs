use crate::structs::ConnectionState::Disconnected;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
/// Defines the states a Cwtch connection can be in
pub enum ConnectionState {
    /// The Cwtch connection is not conected at all
    Disconnected,
    /// Cwtch is attempting to connect to the address, it may or may not be online
    Connecting,
    /// Cwtch has made a basic connection to the address, but not done authorization. The address is online but unverified as the desired target
    Connected,
    /// Cwtch has authenticated the desired connection
    Authenticated,
    /// In the case of a server connection, Cwtch has finished syncing messages from the server and is caught up
    Synced,
    /// The connection attempt failed
    Failed,
    /// The connection has been killed
    Killed,
}

impl Default for ConnectionState {
    fn default() -> ConnectionState {
        Disconnected
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
/// Defines the various authorization modes a contact can be in
pub enum ContactAuthorization {
    /// This is an unknown (new?) contact. The user has not approved them
    Unknown,
    /// The contact is approved by the user (manual action)
    Approved,
    /// The contact is blocked by the user, should be ignored
    Blocked,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
#[allow(non_snake_case)]
/// Struct to serialize/deserialize events coming off the Cwtch appbus
pub struct CwtchEvent {
    /// the type of event, as defined in https://git.openprivacy.ca/cwtch.im/cwtch/src/branch/master/event/common.go
    pub event_type: String,
    /// event ID that can be used to respond to some events and keep them differentiated (event_ID because golang naming conventions in libCwtch-go)
    pub event_ID: String,
    /// a map of keys and values of arguments for the event as defined in https://git.openprivacy.ca/cwtch.im/cwtch/src/branch/master/event/common.go
    pub data: HashMap<String, String>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Struct to serialize/deserialize contacts coming from libcwtch-go
pub struct Contact {
    /// onion address / id of the contact
    pub onion: String,
    /// display name of the contact, as determined in libcwtch-go from name specified by contact
    pub name: String,
    #[serde_as(deserialize_as = "DefaultOnError")]
    // cwtch loads profile/contacts from storage and leaves status blank, it's filled in "soon" by events...
    /// contact connection status
    pub status: ConnectionState,
    /// contact authorization state as set by profile
    pub authorization: ContactAuthorization,
    /// is this contact a group? if so "onion" will be a group ID
    pub is_group: bool,
    //attr: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
/// Struct to serialize/deserialize servers coming from libcwtch-go
pub struct Server {
    /// onion address of the server
    pub onion: String,
    /// server connection status
    pub status: ConnectionState,
}

#[derive(Debug)]
/// Struct to serialize/deserialize profiles coming from libcwtch-go
pub struct Profile {
    /// onion address / ID of the profile
    pub onion: String,
    /// nick name of the onion as supplied by libcwtch-go based on "name" attribute
    pub nick: String,
    /// path to a profile image, controled by "picture" attribute
    pub image_path: String,
    /// all profile attributes
    pub attr: HashMap<String, String>,
    /// map of contacts [ onion => contact ]
    pub contacts: HashMap<String, Contact>,
    /// map of servers [ onion => server ]
    pub servers: HashMap<String, Server>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Struct to serialize/deserialize messages sent over Cwtch between profiles / contacts
pub struct Message {
    /// overlay id that the message is targeting as defined in cwtch/model/overlay.go
    /// [  OverlayChat = 1, OverlayInviteContact = 100, OverlayInviteGroup = 101, OverlayFileSharing = 200 ]
    pub o: i64,
    /// data of the message
    pub d: String,
}

impl Profile {
    /// Create a new profile populated from supplied data
    ///   contacts_json as supplied by libcwtch-go, a map of contacts
    ///   server_list as supplied by libcwtch-go, a map of servers
    pub fn new(
        identity: &str,
        name: &str,
        picture: &str,
        contacts_json: &str,
        server_list: &str,
    ) -> Profile {
        let contacts = Profile::process_contacts(contacts_json);
        let servers = Profile::process_servers(server_list);
        Profile {
            onion: identity.to_string(),
            nick: name.to_string(),
            image_path: picture.to_string(),
            attr: Default::default(),
            contacts: contacts,
            servers: servers,
        }
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
