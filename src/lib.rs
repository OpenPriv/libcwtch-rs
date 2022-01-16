#![doc = include_str!( "../README.md" )]
#![doc(html_logo_url = "https://git.openprivacy.ca/cwtch.im/cwtch-ui/media/branch/trunk/cwtch.png")]
#![doc(html_root_url = "https://git.openprivacy.ca/cwtch.im/libcwtch-rs")]
#![deny(missing_docs)]

mod bindings_go;
mod cwtchlib_go;

/// Basic structs using data from Cwtch and for deserializing JSON and serializing to JSON to communicate with Cwtch
pub mod structs;

/// Interface to a Cwtch app with API matching libcwtch
pub trait CwtchLib {
    /// Start a cwtch application using app_dir to store all user profile data and looking to tor_path to find tor to run
    fn start_cwtch(&self, app_dir: &str, tor_path: &str) -> i32;

    /// Send json of a structs::CwtchEvent to the cwtch app bus
    fn send_app_event(&self, event_json: &str);

    /// Send json of a structs::CwtchEvent to a cwtch profile
    fn send_profile_event(&self, profile: &str, event_json: &str);

    /// Pull json of a structs::CwtchEvent off the appbus for responding to
    fn get_appbus_event(&self) -> String;

    /// Create a new profile encrypted with pass
    fn create_profile(&self, nick: &str, pass: &str);

    /// Load any profiles encrypted by pass
    fn load_profiles(&self, pass: &str);

    /// Cause profile to accept conversation
    fn accept_conversation(&self, profile: &str, conversation_id: i32);

    /// Cause profile to block conversation
    fn block_contact(&self, profile: &str, conversation_id: i32);

    /// Cause profile to unblock contact
    fn unblock_contact(&self, profile: &str, conversation_id: i32);

    /// Get a specific message for conversation of profile by index
    fn get_message(&self, profile: &str, conversation_id: i32, message_index: i32) -> String;

    /// Get a specific message for a conversation by its id
    fn get_message_by_id(&self, profile: &str, conversation_id: i32, message_id: i32) -> String;

    /// Get a specific message for conversation of profile by hash
    fn get_message_by_content_hash(&self, profile: &str, conversation_id: i32, hash: &str) -> String;

    /// Send json of a structs::Message from profile to contact
    fn send_message(&self, profile: &str, conversation_id: i32, msg: &str);

    /// Send profile's contact an invite for/to target
    fn send_invitation(&self, profile: &str, conversation_id: i32, target_id: i32);

    /// share a file file_path with a conersation
    fn share_file(&self, profile: &str, conversation_id: i32, file_path: &str);

    /// download a file from a conversation to the file_path
    fn download_file(&self, profile: &str, conversation_id: i32, file_path: &str, manifest_path: &str, file_key: &str);

    /// Query the status of a download
    fn check_download_status(&self, profile: &str, file_key: &str);

    /// Verufy a download is done, and if not, resume it
    fn verify_or_resume_download(&self, profile: &str, conversation_id: i32, file_key: &str);

    /// Ask the ACN inside the Cwtch app to restart the tor connection
    fn reset_tor(&self);

    /// Cause profile to create a group on server with name
    fn create_group(&self, profile: &str, server: &str, name: &str);

    /// Delete profile with encryption/password check of pass
    fn delete_profile(&self, profile: &str, pass: &str);

    /// Cause profile to archive conversation with contact
    fn archive_conversation(&self, profile: &str, conversation_id: i32);

    /// Cause profile to delete contact/group identified by handle
    fn delete_contact(&self, profile: &str, conversation_id: i32);

    /// Cuase profile to attempt to import a contact/group/keybundle identified by bundle
    fn import_bundle(&self, profile: &str, bundle: &str);

    /// Set a profile attribute key to val
    fn set_profile_attribute(&self, profile: &str, key: &str, val: &str);

    /// Set a profile's contact's attribute of key to val
    fn set_conversation_attribute(&self, profile: &str, conversation_id: i32, key: &str, val: &str);

    /// Set an attribute on a message in a conversation
    fn set_message_attribute(&self, profile: &str, conversation_id: i32, channel_id: i32, message_id: i32, attribute_key: &str, attribute_value: &str);

    /// Change a profile's password to new_pass if old_pass is correct
    fn change_password(&self, profile: &str, old_pass: &str, new_pass: &str, new_pass_again: &str);

    /// Shutdown the cwtch app and associated ACN
    fn shutdown_cwtch(&self);

    /// Server functions require server experiment to be enabled

    /// Load all servers encrypted by password
    fn load_servers(&self, password: &str);

    /// Create a new server, encrypted with password, autostart i8 used as bool
    fn create_server(&self, password: &str, description: &str, autostart: i8);

    /// Delete the specified server (if password is correct)
    fn delete_server(&self, onion: &str, current_password: &str);

    /// Launch all loaded servers
    fn launch_servers(&self);

    /// Launch the specified server
    fn launch_server(&self, onion: &str);

    /// Stop the specified server
    fn stop_server(&self, onion: &str);

    /// Stop all running servers
    fn stop_servers(&self);

    /// Destroy all servers leaving htem un-re-runnable. call only on shutdown
    fn destroy_servers(&self);

    /// Set the specified server's attribute of key to val
    fn set_server_attribute(&self, onion: &str, key: &str, val: &str);
}

/// Create a new CwtchLib that is backed by bindings to libcwtch-go
pub fn new_cwtchlib_go() -> impl CwtchLib {
    bindings_go::CwtchLibGo {}
}
