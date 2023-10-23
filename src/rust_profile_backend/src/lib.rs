use ic_cdk::{
    export::{
        candid::{CandidType, Deserialize},
        Principal,
    },
    query, update,
};
use std::cell::RefCell;
use std::collections::BTreeMap;

type IdStore = BTreeMap<String, Principal>;
type ProfileStore = BTreeMap<Principal, Profile>;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Profile {
    pub name: String,
    pub age: u16,
    pub details: Vec<String>,
}

thread_local! {
    static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
    static ID_STORE: RefCell<IdStore> = RefCell::default();
}

#[query(name = "isRegistered")]
fn is_registered(name: String) -> bool {
    ID_STORE.with(|id_store| {
        let binding = id_store.borrow();
        binding.get(&name).is_some()
    })
} 

#[query(name = "getProfile")]
fn get_profile(name: String) -> Profile {
    ID_STORE.with(|id_store| {
        PROFILE_STORE.with(|profile_store| {
            id_store
                .borrow()
                .get(&name)
                .and_then(|id| profile_store.borrow().get(id).cloned()).unwrap_or_default()
        })
    })
}

#[update]
fn register(profile: Profile) -> bool {
    let res =  is_registered(profile.name.clone());
    if res {
        return false;
    }

    let principal_id = ic_cdk::api::caller();
    ID_STORE.with(|id_store| {
        id_store
            .borrow_mut()
            .insert(profile.name.clone(), principal_id);
    });
    PROFILE_STORE.with(|profile_store| {
        profile_store.borrow_mut().insert(principal_id, profile);
    });

    true
}