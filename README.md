# Inventory CRUD (Rust) - Beginner Friendly Guide

This project is a CLI (terminal) application to manage inventory for:
- Products (finite stock).
- Services (infinite stock, price only).

Data is persisted in `inventory.txt`.

## 1. What the system does

From the menu, you can:
- List items.
- Add a product.
- Add a service.
- Sell an item.
- Update price.
- Delete an item.
- Restock (products only).

## 2. Project structure (easy to follow)

- `src/main.rs`
  - CLI layer only.
  - Reads user input and calls business logic.
- `src/inventory.rs`
  - Business rules and use cases.
  - Core behavior of the application.
- `src/models.rs`
  - Domain models (`Product`, `Service`) and the `Salable` trait.
- `src/errors.rs`
  - Domain-specific errors (`StoryError`).
- `src/storage.rs`
  - File persistence (load/save).
- `src/lib.rs`
  - Exposes modules as a library API.

## 3. End-to-end flow of an operation

Example: selling an item.

1. User selects "Sell item" in `main.rs`.
2. `main.rs` calls `process_sale_and_save(...)` in `inventory.rs`.
3. `inventory.rs` finds the item in memory.
4. The item applies its own sale logic (`make_sale`) based on its type.
5. If successful, the updated state is saved into `inventory.txt`.

## 4. Important business rules

- Invalid prices are rejected (`<= 0`).
- Product sale:
  - If stock is `0` -> `OutOfStock`.
  - If stock is insufficient -> `InsufficientStocks`.
- Service sale:
  - Always allowed (infinite stock model).
  - Cannot be restocked.
- Adding an existing service by name:
  - Updates its price.

## 5. Rust concepts used in this project (simple explanations)

## 5.1 Trait-based polymorphism

What it is:
- A trait is a contract of methods.

Where:
- `Salable` in `models.rs`.

Why it matters here:
- `Product` and `Service` share one interface:
  - `name`, `price`, `make_sale`, etc.
- `Inventory` can work with both types without duplicated logic.

## 5.2 Dynamic polymorphism (`dyn Trait`)

What it is:
- Storing different concrete types behind one shared trait interface.

Where:
- `Vec<Box<dyn Salable>>` in `inventory.rs`.

Why it matters here:
- The same list can hold products and services together.

## 5.3 Ownership

What it is:
- Every value in Rust has one owner.

Where:
- `add_and_save` receives `Box<dyn Salable>` and moves it into the vector.

Why it matters here:
- Safe memory management without garbage collection.

## 5.4 Borrowing and mutability

What it is:
- `&T` for read-only access.
- `&mut T` for mutable access.

Where:
- Trait methods use `&self` and `&mut self`.
- `iter_mut()` is used for in-place item updates.

Why it matters here:
- You can update stock/price safely with compiler guarantees.

## 5.5 Error handling with `Result` and `?`

What it is:
- `Result<T, E>` models success or failure.
- `?` propagates errors in a concise way.

Where:
- Most methods in `inventory.rs` and `storage.rs`.

Why it matters here:
- File errors and domain validation errors are handled cleanly.

## 5.6 Custom domain errors with `enum`

What it is:
- A strongly-typed list of business errors.

Where:
- `StoryError` in `errors.rs`.

Examples:
- `ProductNotFound`
- `OutOfStock(String)`
- `InvalidPrice`
- `NonStockableItem(String)`

Why it matters here:
- Clear, explicit, and maintainable error logic.

## 5.7 Converting `Option` into `Result`

What it is:
- `.find(...)` returns `Option`.
- `.ok_or(...)` turns missing values into meaningful errors.

Where:
- Item lookups in `inventory.rs`.

Why it matters here:
- Missing items become explicit domain errors (`ProductNotFound`).

## 5.8 Pattern matching (`match`)

What it is:
- A safe and expressive control-flow mechanism.

Where:
- CLI menu handling in `main.rs`.
- Line parsing in `storage.rs` (`PRODUCT` / `SERVICE`).
- Error formatting in `errors.rs` (`Display` implementation).

## 5.9 Iterators and closures

Where:
- `.find(...)`, `.retain(...)`, `.iter()`, `.iter_mut()`.

Why it matters:
- Cleaner and safer collection logic.

## 5.10 Modules and visibility (`pub`)

What it is:
- Code organized by responsibility in separate modules.

Where:
- `lib.rs` exposes public modules.

Why it matters:
- Easier maintenance and easier scaling.

## 6. Service design decisions

- `Service` has no `quantity` field.
- `quantity()` returns `0` as a non-stock placeholder.
- `make_sale()` always returns `Ok(())`.
- `a_csv()` stores real service description and price.

This models the "infinite stock service" behavior clearly.

## 7. Run the project

```bash
cargo run
```

## 8. Validate compilation

```bash
cargo check
```

## 9. Quick glossary

- Trait: behavior contract.
- `dyn Trait`: dynamic dispatch through a trait object.
- Ownership: who owns a value.
- Borrowing: temporary access to a value.
- `Result`: typed success/error outcome.
- `Option`: value present/absent.
- `match`: pattern-based control flow.
