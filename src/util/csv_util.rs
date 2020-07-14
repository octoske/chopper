use crate::chopper::chopper::Source;
use crate::error::{CliResult, Error};
use crate::source::csv_configs::CSVOutputConfig;
use std::collections::HashMap;

pub fn parse_into_delimiter(str: &str) -> CliResult<u8> {
    /* Code in this function was adapted from public domain xsv project. */
    match &*str {
        r"\t" => Ok(b'\t'),
        s => {
            if s.len() != 1 {
                return Err(Error::from(format!(
                    "Error: specified delimiter '{}' is not a single ASCII character.",
                    s
                )));
            }
            let c = s.chars().next().unwrap();
            if c.is_ascii() {
                Ok(c as u8)
            } else {
                Err(Error::from(format!(
                    "Error: specified delimiter '{}' is not an ASCII character.",
                    c
                )))
            }
        }
    }
}

pub fn create_csv_output_config_from_source(
    sources: &mut Vec<Box<dyn Source>>,
    delimiter: &str,
) -> CSVOutputConfig {
    let mut all_sources_have_native_timestamps = true;
    for source in sources {
        if !source.has_native_timestamp_column() {
            all_sources_have_native_timestamps = false;
            break;
        }
    }
    CSVOutputConfig::new(delimiter, all_sources_have_native_timestamps)
}

pub fn guess_delimiter(row: &str, possible_delimiters: &[u8]) -> u8 {
    debug_assert_ne!(possible_delimiters.len(), 0);

    let mut counts: HashMap<u8, u32> = HashMap::new();

    for d in possible_delimiters {
        counts.insert(*d, 0);
    }

    for c in row.chars() {
        counts.entry(c as u8).and_modify(|count| *count += 1);
    }

    let mut sorted = possible_delimiters.to_vec();
    sorted.sort_by(|a, b| counts[b].cmp(&counts[a]));

    *sorted.get(0).unwrap()
}
