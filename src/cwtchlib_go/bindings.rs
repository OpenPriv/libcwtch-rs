/* automatically generated by rust-bindgen 0.59.2 */

#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
pub type size_t = ::std::os::raw::c_ulong;
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(
        ::std::mem::size_of::<max_align_t>(),
        32usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        ::std::mem::align_of::<max_align_t>(),
        16usize,
        concat!("Alignment of ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce2 as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GoString_ {
    pub p: *const ::std::os::raw::c_char,
    pub n: isize,
}
#[test]
fn bindgen_test_layout__GoString_() {
    assert_eq!(
        ::std::mem::size_of::<_GoString_>(),
        16usize,
        concat!("Size of: ", stringify!(_GoString_))
    );
    assert_eq!(
        ::std::mem::align_of::<_GoString_>(),
        8usize,
        concat!("Alignment of ", stringify!(_GoString_))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GoString_>())).p as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GoString_),
            "::",
            stringify!(p)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GoString_>())).n as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_GoString_),
            "::",
            stringify!(n)
        )
    );
}
pub type GoInt8 = ::std::os::raw::c_schar;
pub type GoUint8 = ::std::os::raw::c_uchar;
pub type GoInt16 = ::std::os::raw::c_short;
pub type GoUint16 = ::std::os::raw::c_ushort;
pub type GoInt32 = ::std::os::raw::c_int;
pub type GoUint32 = ::std::os::raw::c_uint;
pub type GoInt64 = ::std::os::raw::c_longlong;
pub type GoUint64 = ::std::os::raw::c_ulonglong;
pub type GoInt = GoInt64;
pub type GoUint = GoUint64;
pub type GoUintptr = ::std::os::raw::c_ulong;
pub type GoFloat32 = f32;
pub type GoFloat64 = f64;
pub type GoComplex64 = __BindgenComplex<f32>;
pub type GoComplex128 = __BindgenComplex<f64>;
pub type _check_for_64_bit_pointer_matching_GoInt = [::std::os::raw::c_char; 1usize];
pub type GoString = _GoString_;
pub type GoMap = *mut ::std::os::raw::c_void;
pub type GoChan = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GoInterface {
    pub t: *mut ::std::os::raw::c_void,
    pub v: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_GoInterface() {
    assert_eq!(
        ::std::mem::size_of::<GoInterface>(),
        16usize,
        concat!("Size of: ", stringify!(GoInterface))
    );
    assert_eq!(
        ::std::mem::align_of::<GoInterface>(),
        8usize,
        concat!("Alignment of ", stringify!(GoInterface))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GoInterface>())).t as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GoInterface),
            "::",
            stringify!(t)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GoInterface>())).v as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GoInterface),
            "::",
            stringify!(v)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GoSlice {
    pub data: *mut ::std::os::raw::c_void,
    pub len: GoInt,
    pub cap: GoInt,
}
#[test]
fn bindgen_test_layout_GoSlice() {
    assert_eq!(
        ::std::mem::size_of::<GoSlice>(),
        24usize,
        concat!("Size of: ", stringify!(GoSlice))
    );
    assert_eq!(
        ::std::mem::align_of::<GoSlice>(),
        8usize,
        concat!("Alignment of ", stringify!(GoSlice))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GoSlice>())).data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GoSlice),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GoSlice>())).len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GoSlice),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GoSlice>())).cap as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(GoSlice),
            "::",
            stringify!(cap)
        )
    );
}
extern "C" {
    pub fn c_StartCwtch(
        dir_c: *mut ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        tor_c: *mut ::std::os::raw::c_char,
        torLen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn c_ReconnectCwtchForeground();
}
extern "C" {
    pub fn c_SendAppEvent(json_ptr: *mut ::std::os::raw::c_char, json_len: ::std::os::raw::c_int);
}
extern "C" {
    pub fn c_SendProfileEvent(
        onion_ptr: *mut ::std::os::raw::c_char,
        onion_len: ::std::os::raw::c_int,
        json_ptr: *mut ::std::os::raw::c_char,
        json_len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_GetAppBusEvent() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn c_CreateProfile(
        nick_ptr: *mut ::std::os::raw::c_char,
        nick_len: ::std::os::raw::c_int,
        pass_ptr: *mut ::std::os::raw::c_char,
        pass_len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_LoadProfiles(
        passwordPtr: *mut ::std::os::raw::c_char,
        passwordLen: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_AcceptConversation(
        profilePtr: *mut ::std::os::raw::c_char,
        profileLen: ::std::os::raw::c_int,
        conversation_id: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_BlockContact(
        profilePtr: *mut ::std::os::raw::c_char,
        profileLen: ::std::os::raw::c_int,
        conversation_id: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_UnblockContact(
        profilePtr: *mut ::std::os::raw::c_char,
        profileLen: ::std::os::raw::c_int,
        conversation_id: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_GetMessage(
        profile_ptr: *mut ::std::os::raw::c_char,
        profile_len: ::std::os::raw::c_int,
        conversation_id: ::std::os::raw::c_int,
        message_index: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn c_GetMessageByID(
        profile_ptr: *mut ::std::os::raw::c_char,
        profile_len: ::std::os::raw::c_int,
        conversation_id: ::std::os::raw::c_int,
        message_index: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn c_GetMessagesByContentHash(
        profile_ptr: *mut ::std::os::raw::c_char,
        profile_len: ::std::os::raw::c_int,
        conversation_id: ::std::os::raw::c_int,
        contenthash_ptr: *mut ::std::os::raw::c_char,
        contenthash_len: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn c_FreePointer(ptr: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn c_SendMessage(
        profile_ptr: *mut ::std::os::raw::c_char,
        profile_len: ::std::os::raw::c_int,
        conversation_id: ::std::os::raw::c_int,
        msg_ptr: *mut ::std::os::raw::c_char,
        msg_len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_SendInvitation(
        profile_ptr: *mut ::std::os::raw::c_char,
        profile_len: ::std::os::raw::c_int,
        conversation_id: ::std::os::raw::c_int,
        target_id: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_ShareFile(
        profile_ptr: *mut ::std::os::raw::c_char,
        profile_len: ::std::os::raw::c_int,
        conversation_id: ::std::os::raw::c_int,
        filepath_ptr: *mut ::std::os::raw::c_char,
        filepath_len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_DownloadFile(
        profile_ptr: *mut ::std::os::raw::c_char,
        profile_len: ::std::os::raw::c_int,
        conversation_id: ::std::os::raw::c_int,
        filepath_ptr: *mut ::std::os::raw::c_char,
        filepath_len: ::std::os::raw::c_int,
        manifestpath_ptr: *mut ::std::os::raw::c_char,
        manifestpath_len: ::std::os::raw::c_int,
        filekey_ptr: *mut ::std::os::raw::c_char,
        filekey_len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_CheckDownloadStatus(
        profilePtr: *mut ::std::os::raw::c_char,
        profileLen: ::std::os::raw::c_int,
        fileKeyPtr: *mut ::std::os::raw::c_char,
        fileKeyLen: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_VerifyOrResumeDownload(
        profile_ptr: *mut ::std::os::raw::c_char,
        profile_len: ::std::os::raw::c_int,
        conversation_id: ::std::os::raw::c_int,
        filekey_ptr: *mut ::std::os::raw::c_char,
        filekey_len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_ResetTor();
}
extern "C" {
    pub fn c_CreateGroup(
        profile_ptr: *mut ::std::os::raw::c_char,
        profile_len: ::std::os::raw::c_int,
        server_ptr: *mut ::std::os::raw::c_char,
        server_len: ::std::os::raw::c_int,
        name_ptr: *mut ::std::os::raw::c_char,
        name_len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_DeleteProfile(
        profile_ptr: *mut ::std::os::raw::c_char,
        profile_len: ::std::os::raw::c_int,
        password_ptr: *mut ::std::os::raw::c_char,
        password_len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_ArchiveConversation(
        profile_ptr: *mut ::std::os::raw::c_char,
        profile_len: ::std::os::raw::c_int,
        conversation_id: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_DeleteContact(
        profile_ptr: *mut ::std::os::raw::c_char,
        profile_len: ::std::os::raw::c_int,
        conversation_id: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_ImportBundle(
        profile_ptr: *mut ::std::os::raw::c_char,
        profile_len: ::std::os::raw::c_int,
        bundle_ptr: *mut ::std::os::raw::c_char,
        bundle_len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_SetProfileAttribute(
        profile_ptr: *mut ::std::os::raw::c_char,
        profile_len: ::std::os::raw::c_int,
        key_ptr: *mut ::std::os::raw::c_char,
        key_len: ::std::os::raw::c_int,
        val_ptr: *mut ::std::os::raw::c_char,
        val_len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_SetConversationAttribute(
        profile_ptr: *mut ::std::os::raw::c_char,
        profile_len: ::std::os::raw::c_int,
        conversation_id: ::std::os::raw::c_int,
        key_ptr: *mut ::std::os::raw::c_char,
        key_len: ::std::os::raw::c_int,
        val_ptr: *mut ::std::os::raw::c_char,
        val_len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_SetMessageAttribute(
        profile_ptr: *mut ::std::os::raw::c_char,
        profile_len: ::std::os::raw::c_int,
        conversation_id: ::std::os::raw::c_int,
        channel_id: ::std::os::raw::c_int,
        message_id: ::std::os::raw::c_int,
        key_ptr: *mut ::std::os::raw::c_char,
        key_len: ::std::os::raw::c_int,
        val_ptr: *mut ::std::os::raw::c_char,
        val_len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_ChangePassword(
        profile_ptr: *mut ::std::os::raw::c_char,
        profile_len: ::std::os::raw::c_int,
        oldpassword_ptr: *mut ::std::os::raw::c_char,
        oldpassword_len: ::std::os::raw::c_int,
        newpassword_ptr: *mut ::std::os::raw::c_char,
        newpassword_len: ::std::os::raw::c_int,
        newpassword_again_ptr: *mut ::std::os::raw::c_char,
        newpassword_again_len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_ShutdownCwtch();
}
extern "C" {
    pub fn c_LoadServers(
        passwordPtr: *mut ::std::os::raw::c_char,
        passwordLen: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_CreateServer(
        passwordPtr: *mut ::std::os::raw::c_char,
        passwordLen: ::std::os::raw::c_int,
        descPtr: *mut ::std::os::raw::c_char,
        descLen: ::std::os::raw::c_int,
        autostart: ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn c_DeleteServer(
        onionPtr: *mut ::std::os::raw::c_char,
        onionLen: ::std::os::raw::c_int,
        currentPasswordPtr: *mut ::std::os::raw::c_char,
        currentPasswordLen: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn c_LaunchServers();
}
extern "C" {
    pub fn c_LaunchServer(onionPtr: *mut ::std::os::raw::c_char, onionLen: ::std::os::raw::c_int);
}
extern "C" {
    pub fn c_StopServer(onionPtr: *mut ::std::os::raw::c_char, onionLen: ::std::os::raw::c_int);
}
extern "C" {
    pub fn c_StopServers();
}
extern "C" {
    pub fn c_DestroyServers();
}
extern "C" {
    pub fn c_SetServerAttribute(
        onionPtr: *mut ::std::os::raw::c_char,
        onionLen: ::std::os::raw::c_int,
        keyPtr: *mut ::std::os::raw::c_char,
        keyLen: ::std::os::raw::c_int,
        valPtr: *mut ::std::os::raw::c_char,
        valLen: ::std::os::raw::c_int,
    );
}
