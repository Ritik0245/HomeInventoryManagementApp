#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, symbol_short, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub quantity: u32,
    pub location: String,
}

#[contracttype]
pub enum InventoryKey {
    Item(u64),
    Count,
}

#[contract]
pub struct HomeInventoryContract;

#[contractimpl]
impl HomeInventoryContract {
    pub fn add_item(env: Env, name: String, quantity: u32, location: String) -> u64 {
        let mut count: u64 = env.storage().instance().get(&InventoryKey::Count).unwrap_or(0);
        count += 1;

        let item = Item {
            id: count,
            name,
            quantity,
            location,
        };

        env.storage().instance().set(&InventoryKey::Item(count), &item);
        env.storage().instance().set(&InventoryKey::Count, &count);
        count
    }

    pub fn get_item(env: Env, id: u64) -> Item {
        env.storage()
            .instance()
            .get(&InventoryKey::Item(id))
            .unwrap_or(Item {
                id: 0,
                name: String::from_str(&env, "Not_Found"),
                quantity: 0,
                location: String::from_str(&env, "N/A"),
            })
    }

    pub fn update_quantity(env: Env, id: u64, new_quantity: u32) {
        let mut item = Self::get_item(env.clone(), id);
        item.quantity = new_quantity;
        env.storage().instance().set(&InventoryKey::Item(id), &item);
    }
}
