// tests/test.rs
use soroban_sdk::{testutils::Env as TestEnv, String};
use sistema_de_ventas::SistemaDeVentas; // Asegúrate de que el nombre del contrato esté correctamente importado.

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_register() {
        let env = TestEnv::default();
        let sistema = SistemaDeVentas;

        // Registrar un nuevo producto
        sistema.register(env.clone(), 1, String::from("Producto A"));

        // Verificar que el producto fue registrado correctamente
        let result = sistema.view(env.clone(), 1);
        assert_eq!(result, Some(String::from("Producto A")));
    }

    #[test]
    fn test_edit() {
        let env = TestEnv::default();
        let sistema = SistemaDeVentas;

        // Registrar un producto
        sistema.register(env.clone(), 1, String::from("Producto A"));
        // Editar el producto
        sistema.edit(env.clone(), 1, String::from("Producto A+"));

        // Verificar que el producto fue editado correctamente
        let result = sistema.view(env.clone(), 1);
        assert_eq!(result, Some(String::from("Producto A+")));
    }

    #[test]
    fn test_delete() {
        let env = TestEnv::default();
        let sistema = SistemaDeVentas;

        // Registrar un producto
        sistema.register(env.clone(), 1, String::from("Producto A"));
        // Eliminar el producto
        sistema.delete(env.clone(), 1);

        // Verificar que el producto fue eliminado correctamente
        let result = sistema.view(env.clone(), 1);
        assert_eq!(result, None); // El producto ya no debería existir
    }

    #[test]
    fn test_view_nonexistent() {
        let env = TestEnv::default();
        let sistema = SistemaDeVentas;

        // Intentar ver un producto que no existe
        let result = sistema.view(env.clone(), 999); // ID 999 no debería existir
        assert_eq!(result, None); // Debería retornar None
    }
}
