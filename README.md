# Shalrath &emsp; [![License]][mit] [![Latest Version]][crates.io] [![Documentation]][docs.rs]

[License]: https://img.shields.io/badge/license-MIT-blue.svg
[mit]: LICENSE

[Latest Version]: https://img.shields.io/crates/v/shalrath.svg
[crates.io]: https://crates.io/crates/shalrath

[Documentation]: https://docs.rs/shalrath/badge.svg
[docs.rs]: https://docs.rs/shalrath

# A rusty, spiky, heat-seeking quake map parser
[`shalrath`](crate) is a rust representation, [`nom`] parser and string serializer for Quake [`map`](https://www.gamers.org/dEngine/quake/QDP/qmapspec.html) files.

It's written in pure Rust, and enforces the use of safe code crate-wide via `#![forbid(unsafe_code)]`.

## Rust Representation
The Rust representation lives in the [`repr`] module,
and is a set of structs that represent the contents of a [`map`](https://www.gamers.org/dEngine/quake/QDP/qmapspec.html) file.

The overall class structure - with some of the more specific innermost types omitted for simplicity - looks something like this:
```markdown
Map
└ Entity (1..*)
  ├ Properties (1..1)
  │ └ Property (0..*)
  └ Brushes (1..1)
    └ Brush (0..*)
      └ BrushPlane (4..*)
```

[`Entity`] is a game object that can contain [`Property`]s and [`Brush`]es.

[`Property`]s are key-value pairs stored as [`String`]s.

[`Brush`]es are convex shapes defined by the intersection of a set of [`TexturePlane`]s - 3D planes with associated texture mapping data.

At least one [`Entity`] - known as the `worldspawn` - must exist in any given [`map`](https://www.gamers.org/dEngine/quake/QDP/qmapspec.html), and represents all of its structural [`Brush`]es.
Structural [`Brush`]es are static geometry with no associated behavior.

In Quake terms, entities are given behavior by assigning them a `classname` property, which is used by the game code to assign an actor class that reads from other properties attached to the object.

These entities are separated into two categories:
- `Point Entities` are [`Entity`]s that have a `classname`, but no [`Brush`]es.
    - These are used to represent actors like the player, enemies, or item pickups.
- `Brush Entities` are  [`Entity`]s that have both a `classname` and [`Brush`]es.
    - These are used to represent special world geometry like moving doors, elevators, and similar.

But, that's just for context, more of which can be found in the [map file spec](https://www.gamers.org/dEngine/quake/QDP/qmapspec.html).

Ultimately, what you do with the Rust representation after parsing data into it is down to needs of your project.
To that end, struct members are public in the case of named fields, and exposed via [`Deref`] for collection wrappers like [`Properties`] and [`Brushes`].

## Parsing
The simplest way to parse a [`map`](https://www.gamers.org/dEngine/quake/QDP/qmapspec.html) file into AST is by way of the [`FromStr`] trait:
```
use shalrath::repr::*;

let map =
    "{\"classname\" \"worldspawn\"\n{\n( 0 1 2 ) ( 3 4 5 ) ( 6 7 8 ) TEXTURE 0 0 0 1 1\n}\n}"
        .parse::<Map>()
        .expect("Failed to parse map");

assert_eq!(
    map,
    Map::new(vec![Entity {
        properties: Properties::new(vec![Property {
            key: "classname".into(),
            value: "worldspawn".into()
        }]),
        brushes: Brushes::new(vec![Brush::new(vec![
            BrushPlane {
                plane: Triangle {
                    v0: Point {
                        x: 0.0,
                        y: 1.0,
                        z: 2.0
                    },
                    v1: Point {
                        x: 3.0,
                        y: 4.0,
                        z: 5.0
                    },
                    v2: Point {
                        x: 6.0,
                        y: 7.0,
                        z: 8.0
                    },
                },
                texture: "TEXTURE".into(),
                texture_offset: TextureOffset::Standard { u: 0.0, v: 0.0 },
                angle: 0.0,
                scale_x: 1.0,
                scale_y: 1.0,
                extension: Extension::Standard,
            }
        ])])
    }])
)
```

For a lower-level alternative, the [`parser`] module contains the [`nom`] functions used by the [`FromStr`] implementations,
which can be used to parse plaintext data into individual AST structs.

Of these, [`parse_map`] is the primary entrypoint, and is equivalent to `str::parse::<Map>()`:
```
use shalrath::parser::repr::parse_map;

let map_string = include_str!("../test_data/abstract-test.map");
let (_, map_ast) = parse_map(map_string).expect("Failed to parse map");
println!("{:#?}", map_ast);
```

## String Serialization
The Rust representation can be serialized back into a text-based [`map`](https://www.gamers.org/dEngine/quake/QDP/qmapspec.html) representation via the [`Display`] or [`ToString`] traits:
```
use shalrath::repr::Map;

let map_string = include_str!("../test_data/abstract-test.map");
let map_ast = map_string.parse::<Map>().expect("Failed to parse map file");
let serialized_map_string = map_ast.to_string();
println!("{}", serialized_map_string);
```

In addition, round-trip parsing the resulting string back into the corresponding AST is a lossless operation,
and is included as a standard part of [`shalrath`](crate)'s integration tests:
```
use shalrath::repr::Map;

let map_string = include_str!("../test_data/abstract-test.map");
let map_ast = map_string.parse::<Map>().expect("Failed to parse map file");
let serialized_map_string = map_ast.to_string();
let roundtrip_map_ast = serialized_map_string.parse::<Map>().expect("Failed to parse map file");
assert_eq!(map_ast, roundtrip_map_ast);
```

## Format Support
Several variants of the base Quake 1 [`map`](https://www.gamers.org/dEngine/quake/QDP/qmapspec.html) format exist that retain the same core structure, but modify how brush planes are encoded.

[`shalrath`](crate) supports these by categorizing them by UV format (represented by the [`TextureOffset`] enum):

| UV Format| Notes                                                                                                                 |
| -------- | --------------------------------------------------------------------------------------------------------------------- |
| Standard | Faces project textures based on the closest world X/Y/Z plane.                                                        |
| Valve    | Faces project textures based on custom U/V axes, allowing for skewing and more accurate texturing of curved surfaces. |

...and brush plane extension data, represented by the [`Extension`] enum:

| Brush Plane Extension | Notes                                                                                            |
| --------------------- | ------------------------------------------------------------------------------------------------ |
| Standard              | Brush planes contain no extra data.                                                              |
| Hexen 2               | Brush planes contain an extra numerical value whose usage is unknown.                            |
| Quake 2               | Brush planes contain `content_flags` and `surface_flags` bitmasks, and a floating point `value`. |

Other formats like `Quake 3` and `Daikatana` exist, but are effectively variants of the above, and will be handled transparently by the parser.

## Serde Support
For cases where serializing and deserializing from non-[`map`](https://www.gamers.org/dEngine/quake/QDP/qmapspec.html) formats is required,
[`shalrath`](crate) includes [`serde::Serialize`] and [`serde::Deserialize`] derives for all types in the [`repr`] module.

These can be enabled by applying the `serde` feature flag to the `shalrath` dependency in `Cargo.toml`.

## Streaming
Currently [`shalrath`](crate) only implements [`complete`](nom#streaming--complete) parsers that expect a full set of input data.

[`streaming`](nom#streaming--complete) implementations are planned, but currently pending further research.
