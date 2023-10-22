# Changelog

### 3.0

_2023_10_21_

**Method name changes**

The following methods have been renamed:

- `when_isnt` is now `when_is_not`
- `when_hasnt` is now `when_has_not`

**YAML and FrontMatter data improvements**

YAML parsing has much improved. When previously YAML parsing only supported `HashMap<String, String>`, as in key: value, and nothing else, then now the YAML parser supports a variety of new data structures such as `usize`, `bool`, `Vec` and `HashMap`. 

### 2.0

_2023-10-15_

**No more `set_store` method.**

Store providers are no longer hardcoded, meaning that you can now use your own providers as long as they implement the `StoreProvider` trait. This change also brings with it a breaking API change, meaning you now have to initiate Siena with `siena(...provider)` instead of `Siena::default().set_store()` that you had to do before.

**Custom ordering with `RecordSortOrder::Custom`**

There is an additional enum for `RecordSortOrder`, which allows you to pass a closure that takes in `a` and `b` and returns a 
custom `Ordered`, so that you can do your own sorting.

**Delete records with the `delete` method**

I can't believe I forgot to implement this previously, but the `StoreProvider` trait now expects a `delete` method implementation, and the `LocalProvider` now also implements it, meaning that you can call `delete()` on the queried items to delete them much in the same way as you'd call `set()` to store/update them.
### 1.3.2

_2022-09-19_

- Added the ability to `sort` records using the system-level `id`.

### 1.3.1

_2022-09-19_

- Added Serde's `Serialize` and `Deserialize` derives to the `Record` struct.
- Added the ability to use `when_*` filtering on the system-level `id` value. 

### 1.3.0

_2022-09-18_

- First release ready for mass consumption.