#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ffi::CStr;
use std::ffi::CString;

use super::CwtchLib;
use crate::cwtchlib_go::bindings;

struct c_str_wrap {
    raw: *mut i8,
    len: i32,
}

impl c_str_wrap {
    pub fn new(str: &str) -> c_str_wrap {
        let cs = match CString::new(str) {
            Ok(s) => s,
            Err(_) => CString::new("").unwrap(),
        };
        c_str_wrap {
            len: cs.as_bytes().len() as i32,
            raw: cs.into_raw(),
        }
    }
}

impl Drop for c_str_wrap {
    fn drop(&mut self) {
        unsafe {
            drop(CString::from_raw(self.raw));
        }
    }
}

// c_bind handles setting up c string arguments and freeing them
// c_bind!( $fn_name ( [ $string_args: &str],*  [ $non_string_args : $type ],* ) $c_function -> $return_type? )
macro_rules! c_bind {
    // No return
    ($func_name:ident ($($str1:ident: &str),* ; $($args:ident: $t:ty),* ; $($str2:ident: &str),*) $bind_fn:ident) => {
        fn $func_name(&self,  $($str1: &str, )* $($args: $t, )* $($str2: &str, )*) {
            $(let $str1 = c_str_wrap::new($str1);)*
            $(let $str2 = c_str_wrap::new($str2);)*
            unsafe {
                bindings::$bind_fn($( $str1.raw, $str1.len, )* $($args,)* $( $str2.raw, $str2.len, )*);
            }
        }
    };
    // String return
    ($func_name:ident ($($str1:ident: &str),* ; $($args:ident: $t:ty),* ; $($str2:ident: &str),*) $bind_fn:ident -> String) => {
        fn $func_name(&self,  $($str1: &str, )* $($args: $t, )* $($str2: &str, )*) -> String {
            $(let $str1 = c_str_wrap::new($str1);)*
            $(let $str2 = c_str_wrap::new($str2);)*
            unsafe {
                let result_ptr = bindings::$bind_fn($( $str1.raw, $str1.len, )* $($args,)* $( $str2.raw, $str2.len, )*);
                let result = match CStr::from_ptr(result_ptr).to_str() {
                    Ok(s) => s.to_owned(),
                    Err(_) => "".to_string()
                };
                // return ownership of string memory and call the library to free it
                bindings::c_FreePointer(result_ptr);
                result
            }
        }
    };
    // Non String return
    ($func_name:ident ($($str1:ident: &str),* ; $($args:ident: $t:ty),* ; $($str2:ident: &str),*) $bind_fn:ident -> $bind_fn_ty:ty) => {
        fn $func_name(&self,  $($str1: &str, )* $($args: t, )* $($str2: &str, )*) -> $bind_fn_ty {
            $(let $str1 = c_str_wrap::new($str1);)*
            $(let $str2 = c_str_wrap::new($str2);)*
            unsafe {
                let result = bindings::$bind_fn($( $str1.raw, $str1.len, )* $($args,)* $( $str2.raw, $str2.len, )*);
                result
            }
        }
    };
}

pub struct CwtchLibGo {}

impl CwtchLib for CwtchLibGo    {
    c_bind!(start_cwtch(app_dir: &str, tor_path: &str;;) c_StartCwtch -> i32);
    c_bind!(send_app_event(event_json: &str;;) c_SendAppEvent);
    c_bind!(send_profile_event(profile: &str, event_jason: &str;;) c_SendProfileEvent);
    c_bind!(get_appbus_event(;;) c_GetAppBusEvent -> String);
    c_bind!(create_profile(nick: &str, pass: &str;;) c_CreateProfile);
    c_bind!(load_profiles(pass: &str;;) c_LoadProfiles);
    c_bind!(accept_conversation(profile: &str ; conversation_id: i32 ;) c_AcceptConversation);
    c_bind!(block_contact(profile: &str ; conversation_id: i32; ) c_BlockContact);
    c_bind!(unblock_contact(profile: &str ; conversation_id: i32; ) c_UnblockContact);
    c_bind!(get_message(profile: &str; conversation_id: i32,  message_index: i32 ;) c_GetMessage -> String);
    c_bind!(get_message_by_id(profile: &str ; conversation_id: i32, message_id: i32 ;) c_GetMessageByID -> String);
    c_bind!(get_message_by_content_hash(profile: &str ; conversation_id: i32 ; hash: &str) c_GetMessagesByContentHash -> String);
    c_bind!(send_message(profile: &str; conversation_id: i32; msg: &str) c_SendMessage);
    c_bind!(send_invitation(profile: &str; conversation_id: i32, target_id: i32;) c_SendInvitation);
    c_bind!(share_file(profile: &str; conversation_id: i32; file_path: &str) c_ShareFile);
    c_bind!(download_file(profile: &str; conversation_id: i32; file_path: &str, manifest_path: &str, file_key: &str) c_DownloadFile);
    c_bind!(check_download_status(profile: &str, file_key: &str;;) c_CheckDownloadStatus);
    c_bind!(verify_or_resume_download(profile: &str; conversation_id: i32; file_key: &str) c_VerifyOrResumeDownload);
    fn reset_tor(&self) {
        unsafe {
            bindings::c_ResetTor();
        }
    }
    c_bind!(create_group(profile: &str, server: &str, name: &str;;) c_CreateGroup);
    c_bind!(delete_profile(profile: &str, pass: &str;;) c_DeleteProfile);
    c_bind!(archive_conversation(profile: &str; conversation_id: i32;) c_ArchiveConversation);
    c_bind!(delete_contact(profile: &str; conversation_id: i32;) c_DeleteContact);
    c_bind!(import_bundle(profile: &str, bundle: &str;;) c_ImportBundle);
    c_bind!(set_profile_attribute(profile: &str, key: &str, val: &str;;) c_SetProfileAttribute);
    c_bind!(set_conversation_attribute(profile: &str; conversation_id: i32; key: &str, val: &str) c_SetConversationAttribute);
    c_bind!(set_message_attribute(profile: &str; conversation_id: i32, channel_id: i32, message_id: i32; key: &str, val: &str) c_SetMessageAttribute);
    c_bind!(change_password(profile: &str, old_pass: &str, new_pass: &str, new_pass_again: &str;;) c_ChangePassword);

    fn shutdown_cwtch(&self) {
        unsafe {
            bindings::c_ShutdownCwtch();
        }
    }

    c_bind!(load_servers(password: &str;;) c_LoadServers);
    c_bind!(create_server(password: &str, description: &str; autostart: i8;) c_CreateServer);
    c_bind!(delete_server(onion: &str, current_password: &str;;) c_DeleteServer);
    c_bind!(launch_servers(;;) c_LaunchServers);
    c_bind!(launch_server(onion: &str;;) c_LaunchServer);
    c_bind!(stop_server(onion: &str;;) c_StopServer);
    c_bind!(stop_servers(;;) c_StopServers);
    c_bind!(destroy_servers(;;) c_DestroyServers);
    c_bind!(set_server_attribute(onion: &str, key: &str, val: &str;;) c_SetServerAttribute);
}
