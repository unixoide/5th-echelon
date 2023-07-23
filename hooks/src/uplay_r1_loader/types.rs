use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CString;
use std::mem::ManuallyDrop;
use std::ptr::null_mut;

use tracing::warn;

// https://github.com/Tron0xHex/uplay-r1-loader/
#[derive(Debug)]
pub struct UplaySave {
    pub slot_id: usize,
    pub name: String,
    pub size: usize,
}

#[repr(C)]
struct UplayKey {
    pub cd_key: *mut i8,
}

#[derive(Debug)]
pub struct UplayFriend {
    pub id: String,
    pub username: String,
}

#[repr(C)]
#[derive(Debug)]
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

#[derive(Debug)]
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

#[repr(C)]
struct Friend {
    id: *mut i8,
    username: *mut c_char,
    unknown1: usize,
    unknown2: usize,
    details: *mut FriendDetails,
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
    pub list: *mut *mut c_void,
    pub ty: ListType,
}

impl From<UplayList> for List {
    fn from(value: UplayList) -> Self {
        match value {
            UplayList::CdKeys(keys) => {
                let mut keys = keys
                    .into_iter()
                    .map(CString::new)
                    .map(std::result::Result::unwrap)
                    .map(CString::into_raw)
                    .map(|cd_key| UplayKey { cd_key })
                    .map(Box::new)
                    .map(Box::into_raw)
                    .collect::<Vec<*mut UplayKey>>();
                keys.shrink_to_fit();
                if keys.capacity() > keys.len() {
                    warn!("Capacity still greater than len, memory leak will happen!");
                }
                let mut keys = ManuallyDrop::new(keys);
                List {
                    count: keys.len(),
                    list: keys.as_mut_ptr().cast::<*mut c_void>(),
                    ty: ListType::CdKeys,
                }
            }
            UplayList::Saves(saves) => {
                let mut saves = saves
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
                saves.shrink_to_fit();
                if saves.capacity() > saves.len() {
                    warn!("Capacity still greater than len, memory leak will happen!");
                }
                let mut saves = ManuallyDrop::new(saves);
                List {
                    count: saves.len(),
                    list: saves.as_mut_ptr().cast::<*mut c_void>(),
                    ty: ListType::Saves,
                }
            }
            UplayList::Friends(friends) => {
                let mut friends = friends
                    .into_iter()
                    .map(|f| {
                        Ok::<Friend, std::ffi::NulError>(Friend {
                            id: CString::new(f.id)?.into_raw(),
                            username: CString::new(f.username)?.into_raw(),
                            unknown1: 0,
                            unknown2: 0,
                            details: Box::into_raw(Box::new(FriendDetails {
                                unknown1: 0,
                                unknown2: null_mut(),
                                unknown3: 0,
                                unknown4: null_mut(),
                            })),
                        })
                    })
                    .map(std::result::Result::unwrap)
                    .map(Box::new)
                    .map(Box::into_raw)
                    .collect::<Vec<*mut Friend>>();
                friends.shrink_to_fit();
                if friends.capacity() > friends.len() {
                    warn!("Capacity still greater than len, memory leak will happen!");
                }
                let mut friends = ManuallyDrop::new(friends);
                List {
                    count: friends.len(),
                    list: friends.as_mut_ptr().cast::<*mut c_void>(),
                    ty: ListType::Friends,
                }
            }
        }
    }
}

pub enum UplayList {
    CdKeys(Vec<String>),
    Saves(Vec<UplaySave>),
    Friends(Vec<UplayFriend>),
}

impl TryFrom<List> for UplayList {
    type Error = anyhow::Error;

    fn try_from(value: List) -> ::std::result::Result<Self, Self::Error> {
        anyhow::ensure!(!value.list.is_aligned(), "list not aligned");
        let res = match value.ty {
            ListType::CdKeys => UplayList::CdKeys(
                (0..value.count)
                    .map(|i| unsafe {
                        let ptr = *value.list.add(i);
                        *value.list.add(i) = std::ptr::null_mut();
                        ptr
                    })
                    .map(|item: *mut c_void| {
                        anyhow::ensure!(!item.is_null(), "item is null");
                        let item = item.cast::<UplayKey>();
                        anyhow::ensure!(!item.is_aligned(), "item is not aligned");
                        let owned = unsafe { Box::from_raw(item) };
                        anyhow::ensure!(!owned.cd_key.is_null(), "item.name is null");
                        anyhow::ensure!(!owned.cd_key.is_aligned(), "item.name is not aligned");
                        let s = unsafe { CString::from_raw(owned.cd_key) };
                        s.into_string().map_err(anyhow::Error::from)
                    })
                    .collect::<anyhow::Result<_>>()?,
            ),
            ListType::Saves => UplayList::Saves(
                (0..value.count)
                    .map(|i| unsafe {
                        let ptr = *value.list.add(i);
                        *value.list.add(i) = std::ptr::null_mut();
                        ptr
                    })
                    .map(|item: *mut c_void| {
                        anyhow::ensure!(!item.is_null(), "item is null");
                        let item = item.cast::<Save>();
                        anyhow::ensure!(!item.is_aligned(), "item is not aligned");
                        let item = unsafe { Box::from_raw(item) };
                        anyhow::ensure!(!item.name.is_null(), "item.name is null");
                        anyhow::ensure!(!item.name.is_aligned(), "item.name is not aligned");
                        let s = unsafe { CString::from_raw(item.name) };
                        Ok(UplaySave {
                            slot_id: item.slot_id,
                            name: s.into_string().map_err(anyhow::Error::from)?,
                            size: item.size,
                        })
                    })
                    .collect::<anyhow::Result<_>>()?,
            ),
            ListType::Friends => UplayList::Friends(
                (0..value.count)
                    .map(|i| unsafe {
                        let ptr = *value.list.add(i);
                        *value.list.add(i) = std::ptr::null_mut();
                        ptr
                    })
                    .map(|item: *mut c_void| {
                        anyhow::ensure!(!item.is_null(), "item is null");
                        let item = item.cast::<Friend>();
                        anyhow::ensure!(!item.is_aligned(), "item is not aligned");
                        let item = unsafe { Box::from_raw(item) };
                        anyhow::ensure!(!item.id.is_null(), "item.id is null");
                        anyhow::ensure!(!item.id.is_aligned(), "item.id is not aligned");
                        anyhow::ensure!(!item.username.is_null(), "item.username is null");
                        anyhow::ensure!(
                            !item.username.is_aligned(),
                            "item.username is not aligned"
                        );

                        let id = unsafe { CString::from_raw(item.id) };
                        let username = unsafe { CString::from_raw(item.username) };
                        let id = id.into_string().map_err(anyhow::Error::from)?;
                        let username = username.into_string().map_err(anyhow::Error::from)?;
                        Ok(UplayFriend { id, username })
                    })
                    .collect::<anyhow::Result<_>>()?,
            ),
        };
        match value.ty {
            ListType::CdKeys => {
                unsafe {
                    Vec::from_raw_parts(
                        value.list.cast::<*mut UplayKey>(),
                        value.count,
                        value.count,
                    );
                    *value.list = std::ptr::null_mut();
                };
            }
            ListType::Saves => {
                unsafe {
                    Vec::from_raw_parts(value.list.cast::<*mut Save>(), value.count, value.count);
                    *value.list = std::ptr::null_mut();
                };
            }
            ListType::Friends => {
                unsafe {
                    Vec::from_raw_parts(value.list.cast::<*mut Friend>(), value.count, value.count);
                    *value.list = std::ptr::null_mut();
                };
            }
        }
        Ok(res)
    }
}
