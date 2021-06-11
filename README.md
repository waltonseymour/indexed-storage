# Indexed Storage

Fast and lightweight indexed binary data storage

### Install
```
[dependencies]
indexed_storage = "0.1.0"
```


### Example
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

fn main() {
    let mut writer = indexed_storage::new_writer("db");

    for i in 0..100000 {
        let entry = Entity {
            x: i as f32,
            y: (i as f32).sqrt(),
        };

        let encoded: Vec<u8> = bincode::serialize(&entry).unwrap();
        writer.write_all(&encoded).expect("could not write to db");
    }

    let mut reader = indexed_storage::new_reader("db");

    let encoded = reader.read(8).expect("could not read from db");

    let entry: Entity = bincode::deserialize(&encoded).unwrap();

    // will print:
    // Entity { x: 8.0, y: 2.828427 }
    println!("{:?}", entry);
}

```
