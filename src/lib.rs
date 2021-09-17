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

    /// Pull json of a structs::CwtchEvent off the appbus for responding to
    fn get_appbus_event(&self) -> String;

    /// Create a new profile encrypted with pass
    fn create_profile(&self, nick: &str, pass: &str);

    /// Load any profiles encrypted by pass
    fn load_profiles(&self, pass: &str);

    /// Cause profile to accept contact
    fn accept_contact(&self, profile: &str, contact: &str);

    /// Cause profile to reject contact
    fn reject_invite(&self, profile: &str, contact: &str);

    /// Cause profile to block contact
    fn block_contact(&self, profile: &str, contact: &str);

    /// Cause profile to update contact's message to have it's flags updated
    fn update_message_flags(
        &self,
        profile: &str,
        contact: &str,
        message_id: i32,
        message_flags: u64,
    );

    /// Get a specific message for contact of profile by index
    fn get_message(&self, profile: &str, contact: &str, message_index: i32) -> String;

    /// Get a specific message for contact of profile by hash
    fn get_message_by_content_hash(&self, profile: &str, contact: &str, hash: &str) -> String;

    /// Send json of a structs::Message from profile to contact
    fn send_message(&self, profile: &str, contact: &str, msg: &str);

    /// Send profile's contact an invite for/to target
    fn send_invitation(&self, profile: &str, contact: &str, target: &str);

    /// Ask the ACN inside the Cwtch app to restart the tor connection
    fn reset_tor(&self);

    /// Cause profile to create a group on server with name
    fn create_group(&self, profile: &str, server: &str, name: &str);

    /// Delete profile with encryption/password check of pass
    fn delete_profile(&self, profile: &str, pass: &str);

    /// Cause profile to archive conversation with contact
    fn archive_conversation(&self, profile: &str, contact: &str);

    /// Cause profile to delete contact/group identified by handle
    fn delete_contact(&self, profile: &str, handle: &str);

    /// Cuase profile to attempt to import a contact/group/keybundle identified by bundle
    fn import_bundle(&self, profile: &str, bundle: &str);

    /// Set a profile attribute key to val
    fn set_profile_attribute(&self, profile: &str, key: &str, val: &str);

    /// Set a profile's contact's attribute of key to val
    fn set_contact_attribute(&self, profile: &str, contact: &str, key: &str, val: &str);

    /// Set a profile's group's attribute of key to val
    fn set_group_attribute(&self, profile: &str, group: &str, key: &str, val: &str);

    /// Shutdown the cwtch app and associated ACN
    fn shutdown_cwtch(&self);
}

/// Create a new CwtchLib that is backed by bindings to libcwtch-go
pub fn new_cwtchlib_go() -> impl CwtchLib {
    bindings_go::CwtchLibGo {}
}
