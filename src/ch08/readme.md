# Chapter 08 - Common Collections

- `Vec<T>` is a vector for storing more than one value of same type next to each other (on heap)
    * Implemented using generics
    * `vec!` macro for easy init
    * Adding a new element at the end might require re-allocating memory

    ```rust
    let w = vec![2, 6, 8, 4];   // using macro
    println!("{}", w[1]);

    let mut v = Vec::new();     // using new
    v.push(50);
    v.push(60);

    // using get to retrieve value at an index
    match v.get(1) {
        Some(i) => println!("{}", i),
        None => println!("none")
    }
    // modifying vector elements in for loop
    for i in &mut v {
        *i = *i + 1;
    }
    ```

- Only one string type in core language, i.e. `str`

- `String` type is provided by standard library:
    * Growable, mutable, owned and UTF-8 encoded
    * Other string types - `OsString` or `OsStr`, `CString` or `CStr`
    * `to_string()` method available for all types implementing `Display` trait
    * `push_str()` method or `+` operator or `format!()` macro for extending string
    * Indexing into string is not allowed but using `range` is - bytes vs scalar values vs grapheme clusters

    ```rust
    let mut s1 = "hello".to_string();
    s1 = s1 + " there";     // s1 is moved here
    println!("{}", &s1[0..1]);  // indexing is not allowed but range is

    let mut s2 = String::from("hello");
    s2 = format!("{} there", s2);
    // iterating over unicode scalar values
    for i in s2.chars() {
        println!("{}", i);
    }
    
    let mut s3 = String::new();
    s3.push_str("hello");
    // iterating over bytes
    for i in s3.bytes() {
        println!("{}", i);
    }
    ```

- `HasMap<K,V>` is a hash map storing a mapping of keys of type `K` and value of type `V`
    * Need to bring in to scope using `use`
    * All keys must have the same type and so must the values
    * Default implementation using SipHash (not too fast, but safe from DoS attack)

    ```rust
    use std::collections::HashMap;

    // usual method
    let mut scores = HashMap::new();
    scores.insert(String::from("a"), 52);
    scores.insert(String::from("b"), 60);
    // overwrites existing value
    scores.insert(String::from("b"), 70);
    // access using get
    match scores.get("a") {
        Some(v) => println!("{}", v),
        None => println!("none")
    }

    // from vector of tuples
    let names = vec![String::from("c"), String::from("d")];
    let values = vec![100, 200];
    let mut name_value: HashMap<String, i32> = names.into_iter().zip(values.into_iter()).collect();
    // does not overwrite if key already exists
    let mut val = name_value.entry(String::from("d")).or_insert(600);
    *val += 100;
    // access using for loop
    for (k, v) in &name_value {
        println!("{}: {}", k, v);
    }
    ```