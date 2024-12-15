use soroban_sdk::{contractimpl, contracttype, Env, Map, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Producto {
    nombre: Symbol,
    precio: u32,
}

pub struct AlmacenProductos;

#[contractimpl]
impl AlmacenProductos {
    pub fn inicializar(env: Env) {
        let productos: Map<Symbol, Producto> = Map::new(&env);
        env.storage().set(&Symbol::new(&env, "productos"), &productos);
    }

    pub fn agregar_producto(env: Env, nombre: Symbol, precio: u32) {
        let mut productos: Map<Symbol, Producto> = env
            .storage()
            .get(&Symbol::new(&env, "productos"))
            .unwrap_or_else(|| Map::new(&env));
        
        let producto = Producto { nombre: nombre.clone(), precio };
        productos.set(nombre, producto);

        env.storage().set(&Symbol::new(&env, "productos"), &productos);
    }

    pub fn obtener_producto(env: Env, nombre: Symbol) -> Option<Producto> {
        let productos: Map<Symbol, Producto> = env
            .storage()
            .get(&Symbol::new(&env, "productos"))
            .unwrap_or_else(|| Map::new(&env));
        
        productos.get(nombre)
    }

    pub fn listar_productos(env: Env) -> Map<Symbol, Producto> {
        env.storage()
            .get(&Symbol::new(&env, "productos"))
            .unwrap_or_else(|| Map::new(&env))
    }
}
