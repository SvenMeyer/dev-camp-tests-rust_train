// Rust feature that would otherwise be off, 
// allowing attempted conversions between types, 
// which is exactly what the JSON parsing is doing.
#![feature(try_from)]
#[macro_use]
extern crate hdk;
// Rust's JSON serializer is called serde.
// The three serde related dependencies all relate to the need 
// to serialize to and from JSON within Zomes.
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
extern crate boolinator;
use boolinator::Boolinator;
use hdk::{
    holochain_core_types::{
        dna::entry_types::Sharing,
        json::JsonString,
        entry::Entry,
        error::HolochainError,
        cas::content::Address,
    },
    error::ZomeApiResult,
};
use holochain_wasm_utils::api_serialization::get_links::GetLinksResult;

// Every struct used as a native_type reference should include all 4 derives, as in the example:
// Serialize and Deserialize come from serde_derive, and DefaultJson comes from holochain_core_types_derive

// #[derive(Serialize, Deserialize, Debug, DefaultJson)]
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
struct Person {
    name: String
}

fn handle_add_person(name: String) -> ZomeApiResult<Address> {

   let person_entry = Entry::App("person".into(), Person{name: name}.into());

   let address = hdk::commit_entry(&person_entry)?;

   Ok(address)
}


define_zome! {
    entries: [
            entry!(
            name: "person",
            description: "A person entry",
            sharing: Sharing::Public,
            native_type: Person,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |person: Person, _validation_data: hdk::ValidationData| {
                // this line uses the 'boolinator' import to take a boolean value
                // (person.name.len() >= 2) and convert it into a Rust "Result" type.
                // It should provide a string if it fails validation, which will be
                // given as the error value back to the caller
                (person.name.len() >= 2)
                    .ok_or_else(|| String::from("Name must be at least 2 characters"))
            }
        )
    ]

    genesis: || { 
        Ok(()) 
    }

    functions: [
            add_person: {
            inputs: |person: String|,
            outputs: |response: ZomeApiResult<Address>|,
            handler: handle_add_person
        }
    ]

    traits: {
        hc_public [add_person]
    }
}
