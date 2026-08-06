use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CString;
use std::mem::ManuallyDrop;
use std::ptr::null_mut;

use tracing::warn;

// https://github.com/Tron0xHex/uplay-r1-loader/
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UplaySave {
    pub slot_id: usize,
    pub name: String,
    pub size: usize,
}

#[repr(C)]
struct UplayKey {
    pub cd_key: *mut i8,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UplayFriend {
    pub id: String,
    pub username: String,
    pub is_online: bool,
    /// Session this friend is in; 0 means none. Where it lands in the C structure is decided
    /// by [`SessionField`].
    pub session_id: u32,
    /// Payload of that session, byte for byte as their game handed it over. Ends up in
    /// `FriendDetails.unknown4`.
    pub session_data: Vec<u8>,
    /// Principal id, for the session search. Where it lands is decided by [`PidField`].
    pub pid: u32,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct UplayOverlapped {
    pub unk: u32,
    pub is_completed: u32,
    pub result: usize,
}

impl UplayOverlapped {
    pub fn set_result(&mut self, result: usize) {
        self.set_completed(true);
        self.result = result;
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.is_completed = u32::from(completed);
    }

    pub fn set_success(&mut self) {
        self.set_result(0);
    }
}

#[allow(dead_code)]
#[repr(usize)]
pub enum UplayEventType {
    FriendsFriendListUpdated = 10000,
    FriendsFriendUpdated,
    FriendsGameInviteAccepted,
    FriendsMenuItemSelected,
    PartyMemberListChanged = 20000,
    PartyMemberUserDataUpdated,
    PartyLeaderChanged,
    PartyGameInviteReceived,
    PartyGameInviteAccepted,
    PartyMemberMenuItemSelected,
    PartyMemberUpdated,
    PartyInviteReceived,
    PartyMemberJoined,
    PartyMemberLeft,
    OverlayActivated = 30000,
    OverlayHidden,
    RewardRedeemed = 40000,
    UserAccountSharing = 50000,
    UserConnectionLost,
    UserConnectionRestored,
}

#[repr(C)]
pub struct UplayEvent {
    pub event_type: UplayEventType,
    pub unknown: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ListType {
    CdKeys,
    Saves,
    Friends,
}

#[repr(C)]
struct Save {
    pub slot_id: usize,
    pub name: *mut c_char,
    pub size: usize,
}

use hooks_config::PidField;
use hooks_config::SessionField;

#[repr(C)]
struct Friend {
    id: *mut i8,
    username: *mut c_char,
    unknown1: usize,
    unknown2: usize,
    details: *mut FriendDetails,
    unknown3: usize,
}

#[repr(C)]
struct FriendDetails {
    unknown1: usize,
    unknown2: *mut c_char,
    unknown3: usize,
    unknown4: *mut c_void,
}

#[repr(C)]
#[derive(Debug)]
pub struct List {
    pub count: usize,
    #[allow(clippy::struct_field_names)]
    pub list: *mut *mut c_void,
    pub ty: ListType,
}

macro_rules! valid_ptr {
    ($e:expr) => {
        anyhow::ensure!(!$e.is_null(), format!("{} is null", stringify!($e)));
        anyhow::ensure!($e.is_aligned(), format!("{} is not aligned", stringify!($e)));
    };
    ($e:expr, $ty:ty) => {{
        let tmp = $e.cast::<$ty>();
        valid_ptr!(tmp);
        tmp
    }};
}

impl List {
    fn into_vec<T>(self) -> anyhow::Result<Vec<Box<T>>> {
        valid_ptr!(self.list);
        let vec = unsafe { Vec::from_raw_parts(self.list.cast::<*mut T>(), self.count, self.count) };

        vec.into_iter()
            .map(|item| {
                valid_ptr!(item);
                unsafe { Ok(Box::from_raw(item)) }
            })
            .collect()
    }

    fn into_cdkeys(self) -> anyhow::Result<Vec<String>> {
        self.into_vec::<UplayKey>()?
            .into_iter()
            .map(|owned| {
                valid_ptr!(owned.cd_key);
                let s = unsafe { CString::from_raw(owned.cd_key) };
                s.into_string().map_err(anyhow::Error::from)
            })
            .collect()
    }

    fn into_saves(self) -> anyhow::Result<Vec<UplaySave>> {
        self.into_vec::<Save>()?
            .into_iter()
            .map(|item| {
                valid_ptr!(item.name);
                let s = unsafe { CString::from_raw(item.name) };
                Ok(UplaySave {
                    slot_id: item.slot_id,
                    name: s.into_string().map_err(anyhow::Error::from)?,
                    size: item.size,
                })
            })
            .collect()
    }

    fn into_friends(self) -> anyhow::Result<Vec<UplayFriend>> {
        self.into_vec::<Friend>()?
            .into_iter()
            .map(|item| {
                valid_ptr!(item.id);
                valid_ptr!(item.username);

                let is_online = if item.details.is_null() {
                    true
                } else {
                    let details = unsafe { Box::from_raw(item.details) };
                    details.unknown1 < 2
                };

                let id = unsafe { CString::from_raw(item.id) };
                let username = unsafe { CString::from_raw(item.username) };
                let id = id.into_string().map_err(anyhow::Error::from)?;
                let username = username.into_string().map_err(anyhow::Error::from)?;
                // Reverse direction: a list coming back from the game. Which field would hold
                // a session is exactly what is being determined here, so nothing is read out
                // of it - this path only needs id, name and online state.
                Ok(UplayFriend {
                    id,
                    username,
                    is_online,
                    session_id: 0,
                    session_data: Vec::new(),
                    pid: 0,
                })
            })
            .collect()
    }

    fn from_vec<T>(mut value: Vec<T>, ty: ListType) -> Self {
        value.shrink_to_fit();
        if value.capacity() > value.len() {
            warn!("Capacity still greater than len, memory leak will happen!");
        }
        let mut value = ManuallyDrop::new(value);
        let count = value.len();
        let list = value.as_mut_ptr().cast();
        List { count, list, ty }
    }
}

impl From<UplayList> for List {
    fn from(value: UplayList) -> Self {
        match value {
            UplayList::CdKeys(keys) => {
                let keys = keys
                    .into_iter()
                    .map(CString::new)
                    .map(std::result::Result::unwrap)
                    .map(CString::into_raw)
                    .map(|cd_key| UplayKey { cd_key })
                    .map(Box::new)
                    .map(Box::into_raw)
                    .collect::<Vec<*mut UplayKey>>();
                List::from_vec(keys, ListType::CdKeys)
            }
            UplayList::Saves(saves) => {
                let saves = saves
                    .into_iter()
                    .map(|s| {
                        Ok::<Save, std::ffi::NulError>(Save {
                            slot_id: s.slot_id,
                            name: CString::new(s.name)?.into_raw(),
                            size: s.size,
                        })
                    })
                    .map(std::result::Result::unwrap)
                    .map(Box::new)
                    .map(Box::into_raw)
                    .collect::<Vec<*mut Save>>();
                List::from_vec(saves, ListType::Saves)
            }
            UplayList::Friends(friends) => {
                let cfg = crate::config::get();
                let feld = cfg.map(|c| c.session_field).unwrap_or_default();
                // Passing the payload along can be switched off separately, so that a
                // counter-test does not need a rebuild.
                let daten = cfg.is_none_or(|c| c.share_session_data);
                let pid_feld = cfg.map(|c| c.pid_field).unwrap_or_default();
                let friends = friends
                    .into_iter()
                    .map(|f| {
                        // Where the friend's session goes is decided at runtime, see
                        // [`SessionField`]. Everything not selected keeps its previous value,
                        // so a wrong guess behaves exactly like before.
                        let session = f.session_id as usize;
                        // The friend's principal id - without it the game has nothing to look
                        // their session up by.
                        let pid = f.pid as usize;
                        Ok::<Friend, std::ffi::NulError>(Friend {
                            id: CString::new(f.id)?.into_raw(),
                            username: CString::new(f.username)?.into_raw(),
                            unknown1: if feld == SessionField::FriendUnknown1 { session } else { 0 },
                            unknown2: if feld == SessionField::FriendUnknown2 {
                                session
                            } else if pid_feld == PidField::FriendUnknown2 {
                                pid
                            } else {
                                0
                            },
                            details: Box::into_raw(Box::new(FriendDetails {
                                unknown1: if f.is_online { 0 } else { 2 }, // >1 is offline?
                                unknown2: if feld == SessionField::DetailsUnknown2Str && session != 0 {
                                    CString::new(session.to_string())?.into_raw()
                                } else {
                                    null_mut() // another string?
                                },
                                unknown3: if feld == SessionField::DetailsUnknown3 {
                                    session
                                } else if pid_feld == PidField::DetailsUnknown3 {
                                    pid
                                } else {
                                    0
                                },
                                // The session payload of that friend, byte for byte as their
                                // game handed it over. Without it the other side does get past
                                // the lookup, but opens a session of its own instead of
                                // joining - the id alone is not enough to enter one.
                                // Deliberately leaked: the game keeps the pointer beyond this
                                // call, exactly like the strings above.
                                unknown4: if daten && !f.session_data.is_empty() {
                                    Box::into_raw(f.session_data.clone().into_boxed_slice()).cast::<c_void>()
                                } else {
                                    null_mut() // only used when fetching, but not after??
                                },
                            })),
                            unknown3: if feld == SessionField::FriendUnknown3 {
                                session
                            } else if pid_feld == PidField::FriendUnknown3 {
                                pid
                            } else {
                                0 // must be 0?
                            },
                        })
                    })
                    .map(std::result::Result::unwrap)
                    .map(Box::new)
                    .map(Box::into_raw)
                    .collect::<Vec<*mut Friend>>();
                List::from_vec(friends, ListType::Friends)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UplayList {
    CdKeys(Vec<String>),
    Saves(Vec<UplaySave>),
    Friends(Vec<UplayFriend>),
}

impl TryFrom<List> for UplayList {
    type Error = anyhow::Error;

    fn try_from(value: List) -> ::std::result::Result<Self, Self::Error> {
        let res = match value.ty {
            ListType::CdKeys => UplayList::CdKeys(value.into_cdkeys()?),
            ListType::Saves => UplayList::Saves(value.into_saves()?),
            ListType::Friends => UplayList::Friends(value.into_friends()?),
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cdkeys_are_same_after_conversion() {
        let expected = UplayList::CdKeys(["1234", "ABCD", "foo", "bar"].into_iter().map(String::from).collect());

        let converted: List = expected.clone().into();
        assert!(matches!(expected, UplayList::CdKeys(ref keys) if keys.len() == converted.count));
        assert_eq!(converted.ty, ListType::CdKeys);

        let converted_back: UplayList = converted.try_into().unwrap();
        assert_eq!(converted_back, expected);
    }

    #[test]
    fn saves_are_same_after_conversion() {
        let expected = UplayList::Saves(vec![
            UplaySave {
                slot_id: 1,
                name: "Save 1".into(),
                size: 123,
            },
            UplaySave {
                slot_id: 2,
                name: "Save 2".into(),
                size: 123,
            },
            UplaySave {
                slot_id: 3,
                name: "Save 3".into(),
                size: 123,
            },
        ]);

        let converted: List = expected.clone().into();
        assert!(matches!(expected, UplayList::Saves(ref saves) if saves.len() == converted.count));
        assert_eq!(converted.ty, ListType::Saves);

        let converted_back: UplayList = converted.try_into().unwrap();
        assert_eq!(converted_back, expected);
    }

    #[test]
    fn friends_are_same_after_conversion() {
        let expected = UplayList::Friends(vec![
            UplayFriend {
                id: "ID 1".into(),
                username: "User 1".into(),
                is_online: true,
            },
            UplayFriend {
                id: "ID 2".into(),
                username: "User 2".into(),
                is_online: false,
            },
        ]);

        let converted: List = expected.clone().into();
        assert!(matches!(expected, UplayList::Friends(ref friends) if friends.len() == converted.count));
        assert_eq!(converted.ty, ListType::Friends);

        let converted_back: UplayList = converted.try_into().unwrap();
        assert_eq!(converted_back, expected);
    }
}
