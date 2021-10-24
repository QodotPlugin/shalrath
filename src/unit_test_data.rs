use crate::repr::{
    Brush, BrushPlane, Brushes, Entity, Extension, Map, Point, Properties, Property, TextureOffset,
    TexturePlane, Triangle,
};

pub fn test_point_in() -> &'static str {
    "( -0 8 0 )"
}

pub fn test_point_out() -> Point {
    Point {
        x: -0.0,
        y: 8.0,
        z: 0.0,
    }
}

pub fn test_plane_in() -> &'static str {
    "( -16 -16 -16 ) ( -16 -15 -16 ) ( -16 -16 -15 )"
}

pub fn test_plane_out() -> Triangle {
    Triangle {
        v0: Point {
            x: -16.0,
            y: -16.0,
            z: -16.0,
        },
        v1: Point {
            x: -16.0,
            y: -15.0,
            z: -16.0,
        },
        v2: Point {
            x: -16.0,
            y: -16.0,
            z: -15.0,
        },
    }
}

pub fn test_texture_plane_in() -> &'static str {
    "[ 1 0 0 1 ]"
}

pub fn test_texture_plane_out() -> TexturePlane {
    TexturePlane {
        x: 1.0,
        y: 0.0,
        z: 0.0,
        d: 1.0,
    }
}

pub fn test_texture_offset_in() -> &'static str {
    "[ 1 0 0 1 ] [ 0 -1 0 1 ]"
}

pub fn test_texture_offset_out() -> TextureOffset {
    TextureOffset::Valve {
        u: TexturePlane {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            d: 1.0,
        },
        v: TexturePlane {
            x: 0.0,
            y: -1.0,
            z: 0.0,
            d: 1.0,
        },
    }
}

pub fn test_brush_plane_in() -> &'static str {
    "( -16 -16 -16 ) ( -16 -15 -16 ) ( -16 -16 -15 ) __TB_empty 0 0 0 1 1"
}

pub fn test_brush_plane_out() -> BrushPlane {
    BrushPlane {
        plane: test_plane_out(),
        texture: "__TB_empty".to_string(),
        texture_offset: crate::repr::TextureOffset::Standard { u: 0.0, v: 0.0 },
        angle: 0.0,
        scale_x: 1.0,
        scale_y: 1.0,
        extension: Extension::Standard,
    }
}

pub fn test_extension_in() -> &'static str {
    "64 8 4.5"
}

pub fn test_extension_out() -> Extension {
    Extension::Quake2 {
        content_flags: 64,
        surface_flags: 8,
        value: 4.5,
    }
}

pub fn test_brush_in() -> String {
    "{\n".to_string() + test_brush_plane_in() + "\n" + test_brush_plane_in() + "\n}"
}

pub fn test_brush_out() -> Brush {
    Brush::new(vec![test_brush_plane_out(), test_brush_plane_out()])
}

pub fn test_brushes_in() -> String {
    test_brush_in() + "\n" + &test_brush_in()
}

pub fn test_brushes_out() -> Brushes {
    Brushes::new(vec![test_brush_out(), test_brush_out()])
}

pub fn test_property_in() -> &'static str {
    "\"foo\" \"bar\""
}

pub fn test_property_out() -> Property {
    Property {
        key: "foo".to_string(),
        value: "bar".to_string(),
    }
}

pub fn test_properties_in() -> &'static str {
    "\"foo\" \"bar\"\n\"baz\" \"decafisbad\""
}

pub fn test_properties_out() -> Properties {
    Properties::new(vec![
        Property {
            key: "foo".to_string(),
            value: "bar".to_string(),
        },
        Property {
            key: "baz".to_string(),
            value: "decafisbad".to_string(),
        },
    ])
}

pub fn test_entity_in() -> String {
    "{\n".to_string() + test_properties_in() + "\n" + &test_brushes_in() + "\n}"
}

pub fn test_entity_out() -> Entity {
    Entity {
        properties: test_properties_out(),
        brushes: test_brushes_out(),
    }
}

pub fn test_map_in() -> String {
    test_entity_in() + "\n" + &test_entity_in()
}

pub fn test_map_out() -> Map {
    Map::new(vec![test_entity_out(), test_entity_out()])
}
