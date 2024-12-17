#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String};

#[contract]
pub struct SistemaDeVentas;

#[contractimpl]
impl SistemaDeVentas {
    // Método para registrar un nuevo producto
    pub fn register(env: Env, item_id: u32, details: String) {
        env.storage().instance().set(&item_id, &details);
    }

    // Método para editar los detalles de un producto existente
    pub fn edit(env: Env, item_id: u32, new_details: String) {
        env.storage().instance().set(&item_id, &new_details);
    }

    // Método para eliminar un producto
    pub fn delete(env: Env, item_id: u32) {
        env.storage().instance().remove(&item_id);
    }
    // Método para ver los detalles de un producto
    pub fn view(env: Env, item_id: u32) -> Option<String> {
        env.storage().instance().get(&item_id)
    }
}