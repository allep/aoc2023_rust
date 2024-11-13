struct AlmanacEntry {
    output_start: i32,
    input_start: i32,
    input_range: i32,
}

fn mapping_logic_impl(x: &i32, map: &Vec<AlmanacEntry>) -> i32 {
    for entry in map {
        if *x >= entry.input_start && *x <= entry.input_start + entry.input_range {
            return entry.output_start + *x - entry.input_start;
        }
    }
    *x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proper_mapping() {
        let input = vec![79, 14, 55, 13];

        let seed_to_soil_almanac: Vec<AlmanacEntry> = vec![
            AlmanacEntry {
                output_start: 50,
                input_start: 98,
                input_range: 2,
            },
            AlmanacEntry {
                output_start: 52,
                input_start: 50,
                input_range: 48,
            },
        ];

        let seed_to_soil_mapping = |x: &i32| -> i32 {
            mapping_logic_impl(x, &seed_to_soil_almanac)
        };

        let output: Vec<_> = input.iter().map(seed_to_soil_mapping).collect();
        assert_eq!(output, vec![81, 14, 57, 13]);
    }
}
