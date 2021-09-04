#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ffi::{CString};
use std::ffi::{CStr};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{CwtchLib};
use crate::gobindings;

struct c_str_wrap {
    raw: *mut i8,
    len: i32
}

impl c_str_wrap {
    pub fn new(str: &str) -> c_str_wrap {
        let cs = match CString::new(str) {
            Ok(s) => s,
            Err(_) => CString::new("").unwrap()
        };
        c_str_wrap { len: cs.as_bytes().len() as i32, raw: cs.into_raw() }
    }
}

impl Drop for c_str_wrap {
    fn drop(&mut self) {
        unsafe { CString::from_raw(self.raw); }
    }
}

// c_bind handles setting up c string arguments and freeing them
// c_bind!( $fn_name ( [ $string_args ]* ; [ $non_string_args : $type ]* ) $c_function -> $return_type? )
#[macro_export]
macro_rules! c_bind {
    // macro for returnless fns
    ($func_name:ident ($($str:ident),* ; $($arg:ident: $t:ty),*) $bind_fn:ident) => {
        fn $func_name(&self,  $($str: &str, )* $($arg: $t, )*) {
            $(let $str = c_str_wrap::new($str);)*
            unsafe {
                gobindings::$bind_fn($( $str.raw, $str.len, )* $($arg,)* );
            }
        }
    };
    // macro for str returning fns
    ($func_name:ident ($($str:ident),* ; $($arg:ident: $t:ty),* ) $bind_fn:ident -> String) => {
        fn $func_name(&self,  $($str: &str, )* $($arg: $t, )*) -> String {
            $(let $str = c_str_wrap::new($str);)*
            unsafe {
                let result_ptr = gobindings::$bind_fn($( $str.raw, $str.len, )* $($arg,)* );
                let result = match CStr::from_ptr(result_ptr).to_str() {
                    Ok(s) => s.to_owned(),
                    Err(_) => "".to_string()
                };
                // return ownership of string memory and call the library to free it
                gobindings::c_FreePointer(result_ptr);
                result
            }
        }
    };
    // macro for value returning fns
    ($func_name:ident ($($str:ident),* ; $($arg:ident: $t:ty),* ) $bind_fn:ident -> $bind_fn_ty:ty) => {
        fn $func_name(&self,  $($str: &str, )* $($arg: $t, )*) -> $bind_fn_ty {
            $(let $str = c_str_wrap::new($str);)*
            unsafe {
                let result = gobindings::$bind_fn($( $str.raw, $str.len, )* $($arg,)* );
                result
            }
        }
    };
}

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    EventType: String,
    Data: HashMap<String, String>
}

pub struct GoCwtchLib {}

impl GoCwtchLib {
    c_bind!(send_profile_event(profile, event_json;) c_SendProfileEvent);
}

impl CwtchLib for GoCwtchLib {
    c_bind!(start_cwtch(app_dir, tor_path;) c_StartCwtch -> i32);
    c_bind!(send_app_event(event_json;) c_SendAppEvent);
    c_bind!(get_appbus_event(;) c_GetAppBusEvent -> String);
    c_bind!(create_profile(nick, pass;) c_CreateProfile);
    c_bind!(load_profiles(pass;) c_LoadProfiles);
    c_bind!(accept_contact(profile, contact;) c_AcceptContact);
    c_bind!(reject_invite(profile, contact;) c_RejectInvite);
    c_bind!(block_contact(profile, contact;) c_BlockContact);
    c_bind!(update_message_flags(profile, contact; message_id: i32, message_flags: u64) c_UpdateMessageFlags);
    c_bind!(get_message(profile, contact; message_index: i32) c_GetMessage -> String);
    c_bind!(get_message_by_content_hash(profile, contact, hash;) c_GetMessagesByContentHash -> String);
    c_bind!(send_message(profile, contact, msg;) c_SendMessage);
    c_bind!(send_invitation(profile, contact, target;) c_SendInvitation);
    fn reset_tor(&self) {
        unsafe { gobindings::c_ResetTor(); }
    }
    c_bind!(create_group(profile, server, name;) c_CreateGroup);
    c_bind!(delete_profile(profile, pass;) c_DeleteProfile);
    c_bind!(archive_conversation(profile, contact;) c_ArchiveConversation);
    c_bind!(delete_contact(profile, group;) c_DeleteContact);
    c_bind!(import_bundle(profile, bundle;) c_ImportBundle);
    c_bind!(set_profile_attribute(profile, key, val;) c_SetProfileAttribute);
    c_bind!(set_contact_attribute(profile, contact, key, val;) c_SetContactAttribute);
    c_bind!(set_group_attribute(profile, group, key, val;) c_SetGroupAttribute);

    fn shutdown_cwtch(&self) {
        unsafe { gobindings::c_ShutdownCwtch(); }
    }
}