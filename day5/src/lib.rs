use std::{error::Error, io, process};
use csv::Reader;

#[derive(Debug, serde::Deserialize)]
struct Entry {
    output_start: i32,
    input_start: i32,
    input_range: i32,
}

struct AlmanacEntry {
    output_start: i32,
    input_start: i32,
    input_range: i32,
}

fn mapping_logic_impl(x: i32, map: &Vec<AlmanacEntry>) -> i32 {
    for entry in map {
        if x >= entry.input_start && x <= entry.input_start + entry.input_range {
            return entry.output_start + x - entry.input_start;
        }
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proper_deserialize_to_entry() -> Result<(), Box<dyn Error>> {
        // Note: must be without spaces
        let data = "\
output_start,input_start,input_range
50,98,2
52,50,48
";
        let mut rdr = Reader::from_reader(data.as_bytes());
        println!("Attempt to deserialize");
        for result in rdr.deserialize() {
            println!(" - Read one record");
            let record: Entry = result?;
            println!("{:?}", record);
        }
        Ok(())
    }

    #[test]
    fn proper_mapping() {
        let input = vec![79, 14, 55, 13];

        let ss_map: Vec<AlmanacEntry> = vec![
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

        let sf_map: Vec<AlmanacEntry> = vec![
            AlmanacEntry {
                output_start: 0,
                input_start: 15,
                input_range: 37,
            },
            AlmanacEntry {
                output_start: 37,
                input_start: 52,
                input_range: 2,
            },
            AlmanacEntry {
                output_start: 39,
                input_start: 0,
                input_range: 15,
            },
        ];

        let fw_map: Vec<AlmanacEntry> = vec![
            AlmanacEntry {
                output_start: 49,
                input_start: 53,
                input_range: 8,
            },
            AlmanacEntry {
                output_start: 0,
                input_start: 11,
                input_range: 42,
            },
            AlmanacEntry {
                output_start: 42,
                input_start: 0,
                input_range: 7,
            },
            AlmanacEntry {
                output_start: 57,
                input_start: 7,
                input_range: 4,
            },
        ];

        let wl_map: Vec<AlmanacEntry> = vec![
            AlmanacEntry {
                output_start: 88,
                input_start: 18,
                input_range: 7,
            },
            AlmanacEntry {
                output_start: 18,
                input_start: 25,
                input_range: 70,
            },
        ];

        let lt_map: Vec<AlmanacEntry> = vec![
            AlmanacEntry {
                output_start: 45,
                input_start: 77,
                input_range: 23,
            },
            AlmanacEntry {
                output_start: 81,
                input_start: 45,
                input_range: 19,
            },
            AlmanacEntry {
                output_start: 68,
                input_start: 64,
                input_range: 13,
            },
        ];

        let th_map: Vec<AlmanacEntry> = vec![
            AlmanacEntry {
                output_start: 0,
                input_start: 69,
                input_range: 1,
            },
            AlmanacEntry {
                output_start: 1,
                input_start: 0,
                input_range: 69,
            },
        ];

        let hl_map: Vec<AlmanacEntry> = vec![
            AlmanacEntry {
                output_start: 60,
                input_start: 56,
                input_range: 37,
            },
            AlmanacEntry {
                output_start: 56,
                input_start: 93,
                input_range: 4,
            },
        ];

        let ss_mapper = |x: i32| -> i32 { mapping_logic_impl(x, &ss_map) };

        let sf_mapper = |x: i32| -> i32 { mapping_logic_impl(x, &sf_map) };

        let fw_mapper = |x: i32| -> i32 { mapping_logic_impl(x, &fw_map) };

        let wl_mapper = |x: i32| -> i32 { mapping_logic_impl(x, &wl_map) };

        let lt_mapper = |x: i32| -> i32 { mapping_logic_impl(x, &lt_map) };

        let th_mapper = |x: i32| -> i32 { mapping_logic_impl(x, &th_map) };

        let hl_mapper = |x: i32| -> i32 { mapping_logic_impl(x, &hl_map) };

        let output: Vec<_> = input
            .into_iter()
            .map(ss_mapper)
            .map(sf_mapper)
            .map(fw_mapper)
            .map(wl_mapper)
            .map(lt_mapper)
            .map(th_mapper)
            .map(hl_mapper)
            .collect();

        assert_eq!(output, vec![81, 14, 57, 13]);
    }
}
