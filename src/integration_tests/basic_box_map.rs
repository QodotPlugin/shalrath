use crate::{
    repr::{
        Brush, BrushPlane, Brushes, Entity, Extension, Map, Point, Properties, Property,
        TextureOffset, TrianglePlane,
    },
    test_map,
};

test_map!(
    test_basic_box_map,
    "../../test_data/basic-box.map",
    basic_box_map()
);

fn basic_box_map() -> Map {
    Map::new(vec![Entity {
        properties: Properties::new(vec![Property {
            key: "classname".into(),
            value: "worldspawn".into(),
        }]),
        brushes: Brushes::new(vec![Brush::new(vec![
            BrushPlane {
                plane: TrianglePlane {
                    v0: Point {
                        x: -64.0,
                        y: -64.0,
                        z: -16.0,
                    },
                    v1: Point {
                        x: -64.0,
                        y: -63.0,
                        z: -16.0,
                    },
                    v2: Point {
                        x: -64.0,
                        y: -64.0,
                        z: -15.0,
                    },
                },
                texture: "__TB_empty".into(),
                texture_offset: TextureOffset::Standard { u: 0.0, v: 0.0 },
                angle: 0.0,
                scale_x: 1.0,
                scale_y: 1.0,
                extension: Extension::Standard,
            },
            BrushPlane {
                plane: TrianglePlane {
                    v0: Point {
                        x: -64.0,
                        y: -64.0,
                        z: -16.0,
                    },
                    v1: Point {
                        x: -64.0,
                        y: -64.0,
                        z: -15.0,
                    },
                    v2: Point {
                        x: -63.0,
                        y: -64.0,
                        z: -16.0,
                    },
                },
                texture: "__TB_empty".into(),
                texture_offset: TextureOffset::Standard { u: 0.0, v: 0.0 },
                angle: 0.0,
                scale_x: 1.0,
                scale_y: 1.0,
                extension: Extension::Standard,
            },
            BrushPlane {
                plane: TrianglePlane {
                    v0: Point {
                        x: -64.0,
                        y: -64.0,
                        z: -16.0,
                    },
                    v1: Point {
                        x: -63.0,
                        y: -64.0,
                        z: -16.0,
                    },
                    v2: Point {
                        x: -64.0,
                        y: -63.0,
                        z: -16.0,
                    },
                },
                texture: "__TB_empty".into(),
                texture_offset: TextureOffset::Standard { u: 0.0, v: 0.0 },
                angle: 0.0,
                scale_x: 1.0,
                scale_y: 1.0,
                extension: Extension::Standard,
            },
            BrushPlane {
                plane: TrianglePlane {
                    v0: Point {
                        x: 64.0,
                        y: 64.0,
                        z: 16.0,
                    },
                    v1: Point {
                        x: 64.0,
                        y: 65.0,
                        z: 16.0,
                    },
                    v2: Point {
                        x: 65.0,
                        y: 64.0,
                        z: 16.0,
                    },
                },
                texture: "__TB_empty".into(),
                texture_offset: TextureOffset::Standard { u: 0.0, v: 0.0 },
                angle: 0.0,
                scale_x: 1.0,
                scale_y: 1.0,
                extension: Extension::Standard,
            },
            BrushPlane {
                plane: TrianglePlane {
                    v0: Point {
                        x: 64.0,
                        y: 64.0,
                        z: 16.0,
                    },
                    v1: Point {
                        x: 65.0,
                        y: 64.0,
                        z: 16.0,
                    },
                    v2: Point {
                        x: 64.0,
                        y: 64.0,
                        z: 17.0,
                    },
                },
                texture: "__TB_empty".into(),
                texture_offset: TextureOffset::Standard { u: 0.0, v: 0.0 },
                angle: 0.0,
                scale_x: 1.0,
                scale_y: 1.0,
                extension: Extension::Standard,
            },
            BrushPlane {
                plane: TrianglePlane {
                    v0: Point {
                        x: 64.0,
                        y: 64.0,
                        z: 16.0,
                    },
                    v1: Point {
                        x: 64.0,
                        y: 64.0,
                        z: 17.0,
                    },
                    v2: Point {
                        x: 64.0,
                        y: 65.0,
                        z: 16.0,
                    },
                },
                texture: "__TB_empty".into(),
                texture_offset: TextureOffset::Standard { u: 0.0, v: 0.0 },
                angle: 0.0,
                scale_x: 1.0,
                scale_y: 1.0,
                extension: Extension::Standard,
            },
        ])]),
    }])
}
