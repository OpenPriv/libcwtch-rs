#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod gobindings;
mod bindings_go;

pub mod structs;

pub trait CwtchLib {
    fn start_cwtch(&self, app_dir: &str, tor_path: &str) -> i32;
    fn send_app_event(&self, event_json: &str);
    fn get_appbus_event(&self, ) -> String;
    fn create_profile(&self, nick: &str, pass: &str);
    fn load_profiles(&self, pass: &str);
    fn accept_contact(&self, profile: &str, contact: &str);
    fn reject_invite(&self, profile: &str, contact: &str);
    fn block_contact(&self, profile: &str, contact: &str);
    fn update_message_flags(&self, profile: &str, contact: &str, message_id: i32, message_flags: u64);
    fn get_message(&self, profile: &str, contact: &str, message_index: i32) -> String;
    fn get_message_by_content_hash(&self, profile: &str, contact: &str, hash: &str) -> String;
    fn send_message(&self, profile: &str, contact: &str, msg: &str);
    fn send_invitation(&self, profile: &str, contact: &str, target: &str);
    fn reset_tor(&self, );
    fn create_group(&self, profile: &str, server: &str, name: &str);
    fn delete_profile(&self, profile: &str, pass: &str);
    fn archive_conversation(&self, profile: &str, contact: &str);
    fn delete_contact(&self, profile: &str, group: &str);
    fn import_bundle(&self, profile: &str, bundle: &str);
    fn set_profile_attribute(&self, profile: &str, key: &str, val: &str);
    fn set_contact_attribute(&self, profile: &str, contact: &str, key: &str, val: &str);
    fn set_group_attribute(&self, profile: &str, group: &str, key: &str, val: &str);
    fn shutdown_cwtch(&self, );
}

pub fn new_cwtchlib_go() -> impl CwtchLib {
    bindings_go::GoCwtchLib {}
}

