# Inventory CRUD (Rust) - Explicado Para Principiantes

Este proyecto es una app de consola (CLI) para manejar inventario de:
- Productos (tienen stock).
- Servicios (stock infinito, solo precio).

Los datos se guardan en `inventory.txt`.

## 1. Qué hace el sistema

Desde el menú puedes:
- Listar ítems.
- Agregar producto.
- Agregar servicio.
- Vender ítem.
- Actualizar precio.
- Eliminar ítem.
- Reponer stock (solo productos).

## 2. Estructura del proyecto (fácil de entender)

- `src/main.rs`
  - Solo interfaz CLI.
  - Lee lo que escribe el usuario y llama a la lógica.
- `src/inventory.rs`
  - Reglas del negocio.
  - Aquí vive la “inteligencia” del sistema.
- `src/models.rs`
  - Tipos principales (`Product`, `Service`) y trait `Salable`.
- `src/errors.rs`
  - Errores del dominio (`StoryError`).
- `src/storage.rs`
  - Guardar/cargar desde archivo.
- `src/lib.rs`
  - Expone los módulos para usar el proyecto como librería.

## 3. Flujo completo de una operación

Ejemplo: vender un producto.

1. El usuario elige “Vender ítem” en `main.rs`.
2. `main.rs` llama a `process_sale_and_save(...)` en `inventory.rs`.
3. `inventory.rs` busca el ítem en memoria.
4. El ítem ejecuta su propia regla de venta (`make_sale`) según su tipo.
5. Si sale bien, se guarda todo en `inventory.txt`.

## 4. Reglas de negocio importantes

- Precio inválido: no se permite `<= 0`.
- Producto:
  - Si cantidad = 0 -> `OutOfStock`.
  - Si no alcanza -> `InsufficientStocks`.
- Servicio:
  - Se puede vender siempre (stock infinito).
  - No se puede “reponer stock”.
- Si agregas un servicio con mismo nombre:
  - Se actualiza el precio.

## 5. Rust aplicado en este proyecto (con lenguaje simple)

## 5.1 Trait (polimorfismo)

Qué es:
- Un trait es como un “contrato” de métodos.

Dónde:
- `Salable` en `models.rs`.

Para qué sirve aquí:
- `Product` y `Service` comparten interfaz común:
  - `name`, `price`, `make_sale`, etc.
- Así `Inventory` trabaja con ambos sin duplicar lógica.

## 5.2 Polimorfismo dinámico (`dyn Trait`)

Qué es:
- Guardar distintos tipos concretos bajo una misma interfaz.

Dónde:
- `Vec<Box<dyn Salable>>` en `inventory.rs`.

Para qué sirve aquí:
- La lista puede contener productos y servicios mezclados.

## 5.3 Ownership (propiedad)

Qué es:
- Cada dato en Rust tiene un dueño.

Dónde:
- `add_and_save` recibe `Box<dyn Salable>` y lo mueve al `Vec`.

Para qué sirve aquí:
- Evita referencias colgantes y gestiona memoria de forma segura.

## 5.4 Borrowing (préstamos) y mutabilidad

Qué es:
- `&T` presta para leer.
- `&mut T` presta para modificar.

Dónde:
- Métodos de trait:
  - Lectura con `&self`.
  - Escritura con `&mut self`.
- Búsquedas mutables con `iter_mut()`.

Para qué sirve aquí:
- Actualizar stock/precio sin romper reglas de seguridad.

## 5.5 Manejo de errores con `Result` + `?`

Qué es:
- `Result<T, E>` representa éxito o error.
- `?` propaga errores sin mucho código repetitivo.

Dónde:
- Casi todos los métodos de `inventory.rs` y `storage.rs`.

Para qué sirve aquí:
- Si falla lectura de archivo o una validación de negocio, se corta limpio y se informa.

## 5.6 Errores personalizados con `enum`

Qué es:
- Un enum agrupa varios tipos de error de negocio.

Dónde:
- `StoryError` en `errors.rs`.

Ejemplos reales:
- `ProductNotFound`
- `OutOfStock(String)`
- `InvalidPrice`
- `NonStockableItem(String)`

Ventaja:
- Errores tipados y claros, no solo strings sueltos.

## 5.7 `Option` -> `Result`

Qué es:
- `.find(...)` devuelve `Option`.
- `.ok_or(...)` lo convierte a `Result` con error útil.

Dónde:
- En búsquedas de ítems en `inventory.rs`.

Ventaja:
- Si no existe el ítem, devuelve `ProductNotFound` explícitamente.

## 5.8 `match` (pattern matching)

Qué es:
- Comparación por patrones más segura que muchos `if`.

Dónde:
- Menú en `main.rs`.
- Parseo de líneas en `storage.rs` (`PRODUCT` / `SERVICE`).
- Mensajes de error en `errors.rs` (`Display`).

## 5.9 Iteradores y closures

Dónde:
- `.find(...)`, `.retain(...)`, `.iter()`, `.iter_mut()`.

Ventaja:
- Código más expresivo y menos propenso a errores.

## 5.10 Módulos (`mod`) y visibilidad (`pub`)

Qué es:
- Separar responsabilidades por archivo.

Dónde:
- `lib.rs` declara módulos públicos.

Ventaja:
- Código más mantenible y más fácil de escalar.

## 6. Decisiones de diseño sobre Service

- `Service` no tiene campo `quantity`.
- `quantity()` devuelve `0` porque no aplica stock real.
- `make_sale()` siempre devuelve `Ok(())`.
- `a_csv()` guarda descripción y precio real.

Esto modela correctamente “servicio infinito”.

## 7. Comandos para ejecutar

```bash
cargo run
```

## 8. Comando para validar compilación

```bash
cargo check
```

## 9. Mini glosario rápido

- Trait: contrato de métodos.
- `dyn Trait`: uso dinámico de ese contrato.
- Ownership: quién “posee” un dato.
- Borrowing: préstamo temporal de un dato.
- `Result`: éxito/error tipado.
- `Option`: valor presente o ausente.
- `match`: patrón de control de flujo.
