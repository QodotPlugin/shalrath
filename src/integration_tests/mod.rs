mod basic_box_map;

#[cfg(feature = "non_foss_tests")]
mod non_foss;

#[macro_export]
macro_rules! test_map {
    // Basic case - no comparison data
    ($test_name:tt, $map:expr) => {
        #[test]
        fn $test_name() -> Result<(), Box<dyn std::error::Error>> {
            // Get imported map string slice
            let data = include_str!($map);

            // Parse
            let (_, map) = crate::parser::repr::parse_map(data)?;

            // Ensure it's non-empty
            assert!(!map.is_empty());

            // Ensure the map can make a lossless round trip from AST > String > AST
            assert_eq!(map.to_string().parse::<crate::repr::Map>()?, map);

            Ok(())
        }
    };
    // Advanced case - test against comparison expression
    ($test_name:tt, $map:expr, $cmp:expr) => {
        #[test]
        fn $test_name() -> Result<(), Box<dyn std::error::Error>> {
            // Get imported map string slice
            let data = include_str!($map);

            // Parse
            let (_, map) = crate::parser::repr::parse_map(data)?;

            // Ensure it matches the comparison data
            assert_eq!(map, $cmp);

            // Ensure the map can make a lossless round trip from AST > String > AST
            assert_eq!(map.to_string().parse::<crate::repr::Map>()?, map);
            Ok(())
        }
    };
}

test_map!(
    test_trenchbroom_group_hierarchy,
    "../../test_data/0-trenchbroom-group-hierarchy.map"
);
test_map!(
    test_abstract_test_beveled,
    "../../test_data/abstract_test_beveled.map"
);
test_map!(
    test_abstract_test_edited,
    "../../test_data/abstract_test_edited.map"
);
test_map!(test_abstract_test_a, "../../test_data/abstract_test.map");
test_map!(test_abstract_test_b, "../../test_data/abstract-test.map");
test_map!(test_bevel, "../../test_data/Bevel.map");
test_map!(test_lighting_test, "../../test_data/lighting_test.MAP");
test_map!(
    test_mean_block_single,
    "../../test_data/mean_block_single.map"
);
test_map!(test_mean_block, "../../test_data/mean_block.map");
test_map!(
    test_mean_pillar_single,
    "../../test_data/mean_pillar_single.map"
);
test_map!(test_mean_pillar, "../../test_data/mean_pillar.map");
test_map!(test_test_cylinder, "../../test_data/test_cylinder.map");
test_map!(
    test_trenchbroom_test_daikatana,
    "../../test_data/trenchbroom-test-daikatana.map"
);
test_map!(
    test_trenchbroom_test_q2,
    "../../test_data/trenchbroom-test-q2.map"
);
test_map!(
    test_trenchbroom_test_q3,
    "../../test_data/trenchbroom-test-q3.map"
);
test_map!(
    test_trenchbroom_test_q3_legacy,
    "../../test_data/trenchbroom-test-q3-legacy.map"
);
test_map!(
    test_trenchbroom_test_valve,
    "../../test_data/trenchbroom-test-valve.map"
);
test_map!(test_unit_beveled, "../../test_data/unit_beveled.map");
test_map!(test_unit, "../../test_data/unit.map");
test_map!(test_uv_test, "../../test_data/uv-test.map");
