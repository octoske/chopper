use chrono_tz::America::New_York;
use same_file::is_same_file;

use chopper_lib::chopper::chopper::{ChopperDriver, Source};
use chopper_lib::chopper::header_graph::{HeaderChain, HeaderGraph, HeaderNode};
use chopper_lib::chopper::types::{self, Header};
use chopper_lib::cli::util::YesNoAuto;
use chopper_lib::driver::driver::Driver;
use chopper_lib::error::{self, CliResult};
use chopper_lib::input::input_factory::InputFactory;
use chopper_lib::source::csv_configs::{CSVInputConfig, CSVOutputConfig, OUTPUT_DELIMITER_DEFAULT};
use chopper_lib::source::csv_configs::{TimestampCol, TimestampConfig};
use chopper_lib::write::factory;

#[test]
fn test_decompress() {
    let input = "./tests/input/time_city.csv.gz";
    let inputs = vec![input];
    let output = "./tests/output/output_time_city.csv";
    let ts_config = TimestampConfig::new(TimestampCol::Timestamp(0), None, New_York);
    error::handle_drive_error(test(inputs, output, ts_config));

    assert!(is_same_file(
        "./tests/output/test_decompress.csv",
        "./tests/reference/output_time_city.csv"
    )
    .unwrap());
}

fn test(inputs: Vec<&str>, output: &str, ts_config: TimestampConfig) -> CliResult<()> {
    setup_graph(inputs, output, ts_config)?.drive()
}

fn setup_graph(
    inputs: Vec<&str>,
    output: &str,
    ts_config: TimestampConfig,
) -> CliResult<Box<dyn ChopperDriver>> {
    // source reader and headers
    let input_config = CSVInputConfig::new(None, YesNoAuto::Auto, ts_config)?;
    let mut input_factory = InputFactory::new(None, Some(input_config), None, None)?;
    let mut sources: Vec<Box<dyn Source>> = Vec::new();
    let mut headers: Vec<Header> = Vec::new();
    for i in inputs {
        let source = input_factory.create_source_from_path(i)?;
        headers.push(source.header().clone());
        sources.push(source);
    }

    let csv_output_config = CSVOutputConfig::new(OUTPUT_DELIMITER_DEFAULT, true);
    let header_sink = factory::new_header_sink(Some(output), Some(csv_output_config))?;
    let node_output = HeaderNode::HeaderSink(header_sink);
    let chain = HeaderChain::new(vec![node_output]);

    let graph = HeaderGraph::new(vec![chain]);

    Ok(Box::new(Driver::new(
        sources,
        graph,
        types::TIMESTAMP_RANGE_DEFAULT,
        headers,
    )?))
}
