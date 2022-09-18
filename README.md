# Siena

[![builds.sr.ht status](https://builds.sr.ht/~asko/siena/commits/master/tests.yml.svg)](https://builds.sr.ht/~asko/siena/commits/master/tests.yml?)

Siena is a flat-file (YAML) ORM for Rust, enabling you to easily use flat-file data stores for your application.

## Install

Add the following to your Cargo.toml file:
```TOML
siena = "1.3.0"
```

## Usage

### Setting Store

The first thing you need to do when using Siena, is setting the Store. This will tell Siena to use a correct provider for getting and setting data. 

**Example:**

```rust
use siena::siena::{Store, Siena};

let store = Siena::default().set_store(Store::Local {
    directory: "./path-to-somewhere".to_string()
});
```

Currently, only the `Local` store is supported which enables you to use the local disk for the storage of 
data by giving a specified `directory` to read from, and to write to, but there are plans to enable more Stores in the future, such as S3-compatible services. 

### Fetching Records

Records are placed in collections. A collection is a directory in your store. So let's say 
that you have a collection called "blog-posts", you could fetch them like this:

```rust
let posts = store.collection("blog-posts").get_all();
```

You can also just get the first record via `get_first()` or the last one via 
`.get_last()`.

### Filtering Records

You can filter records using numerous `when_*` methods. And yes, you can chain them 
as much as you want. 

#### `when_is`

To filter records by a record key that equals a given value, you can use the `when_equals` method, like so:

```rust
let posts = store
    .collection("blog-posts")
    .when_is("status", "published")
    .get_all();
```

#### `when_isnt`

Similarly, to filter records the opposite way, by a record key that does _not_ equal a given value, you can use the 
`when_isnt` method: 

```rust
let posts = store
    .collection("blog-posts")
    .when_isnt("status", "published")
    .get_all();
```

#### `when_has`

To filter records by the presence of a record key, you can use the `when_has` method, like so:

```rust
let posts = store
    .collection("blog-posts")
    .when_has("status")
    .get_all();
```

#### `when_hasnt`

Similarly, to filter records the opposite way, by the _lack_ of a presence of a record key, you can use the `when_hasnt` method:

```rust
let posts = store
    .collection("blog-posts")
    .when_hasnt("status")
    .get_all();
```

#### `when_matches`

To filter records by a record key that matches a value according to a Regex pattern, you can use the `when_matches` method, like so:

```rust
let posts = store
    .collection("blog-posts")
    .when_matches("date", r"2022\-09")
    .get_all();
```

There is no opposite method for `when_matches`, because regex gives you the ability to do that yourself.

### Sorting Records

You can sort records with the `sort` method, like so:

```rust
use siena::siena::{RecordSortOrder};

let posts = store
    .collection("blog-posts")
    .sort("date", RecordSortOrder::Desc)
    .get_all();
```

The available ways to sort are:

- `RecordSortOrder::Desc`
- `RecordSortOrder::Asc`

### Limiting Records

To limit the result, use the `limit` method:

```rust
let posts = store
    .collection("blog-posts")
    .limit(10)
    .get_all();
```

### Offsetting Records

To offset the result, use the `offset` method:

```rust
let posts = store
    .collection("blog-posts")
    .offset(10)
    .get_all();
```

### Pagination

With the combination of `limit` and `offset` method, you can create easy pagination, for example:

```rust
let page = 2;
let posts_per_page = 10;

let posts = store
    .collection("blog-posts")
    .offset((page - 1) * posts_per_page)
    .limit(posts_per_page)
    .get_all();
```

Or, simply use the `paginate` method which does this work for you, like this:

```rust
let posts = store
    .collection("blog-posts")
    .paginate(2, 10)
    .get_all();
```

### Updating Records

You can update the result of your query via the `set` method. It doesn't matter if you have one record or multiple records, it will update anything that you have matching your query.

For example:

```rust
let posts = store
    .collection("blog-posts")
    .set(Vec::from([("status", "private")]));
```

This will update all the records in the `blog-post` collection by updating the `status` to `private`.

Whereas this example:

```rust
let posts = store
    .collection("blog-posts")
    .when_is("status", "public")
    .set(Vec::from([("status", "private")]));
```

Will only update all the records that have `status` as `public` _to_ `private`.

### Creating Records

The `create` method is what you use for creating a new record. Note however that the 
record is not persisted until you use the `set` method to add some data. The `set` method is the only method
which writes data. The `create` method only creates the record in-memory so that the `set` method would know 
where to write data.

An example:

```rust
let posts = store
    .create("blog-posts", "hello-world")
    .set(Vec::from([("title", "Hello, World.")]));
```

The `create` method takes two arguments, the collection name, and the ID of the record, which has to be unique to that collection or it will overwrite an existing record.
