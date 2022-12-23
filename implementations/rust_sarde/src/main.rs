extern crate serde;
extern crate serde_json;

use serde_json::Value as JsonValue;
use std::fs;

fn get_scale(metric : &str) -> f64 {
    match metric.chars().next().unwrap() {
        's' => 1.0,
        'm' => 1e-3,
        'u' => 1e-6,
        'n' => 1e-9,
        _   => panic!("unknown metric")
    }
}

fn aggregate_batch(batch: &Vec<JsonValue>) -> f64 {
    let mut sum = 0.0;
    let mut count = 0.0;

    for measurement in batch.iter() {
        let unwrapped = measurement.as_array().unwrap();
        let time = unwrapped[0].as_f64().unwrap();
        let ops = unwrapped[1].as_f64().unwrap();
        sum += time*ops;
        count += ops;
    }

    sum / count
}

fn process_file(raw_filepath: &str, destination_filepath: &str) {
    let input = fs::read_to_string(raw_filepath).unwrap();

    let in_json : JsonValue = serde_json::from_str(&input).expect("should be fine");
    let raw_data = in_json[0]["primaryMetric"]["rawDataHistogram"].as_array().unwrap();
    let metric = in_json[0]["primaryMetric"]["scoreUnit"].as_str().unwrap();
    let scale = get_scale(metric);

    let mut out_json = Vec::new();
    for run in raw_data.iter() {
        let mut new_run_vec = Vec::new();
        for batch in run.as_array().unwrap() {
            let mean = scale * aggregate_batch(batch.as_array().unwrap());
            new_run_vec.push(mean);
        }
        out_json.push(new_run_vec);
    }
    let output = serde_json::to_string(&out_json).unwrap();
    fs::write(destination_filepath, &output).unwrap();
}

fn test_single_file() {
    let mut raw_path = "../../jmh/".to_string();
    let mut dest_path = "/tmp/jmh/".to_string();
    fs::create_dir_all(&dest_path).unwrap();
    let filename = "apache__logging-log4j2#org.apache.logging.log4j.perf.jmh.AsyncLoggersBenchmark.throughput9Params#.json";

    raw_path.push_str(filename);
    dest_path.push_str(filename);

    println!("{}", raw_path);
    process_file(&raw_path, &dest_path);
}

fn process_all_files() {
    let raw_path = "../../jmh/";
    let dest_path = "/tmp/jmh/";
    fs::create_dir_all(&dest_path).unwrap();

    for dir_entry in fs::read_dir(raw_path).unwrap() {
        let path = dir_entry.unwrap().file_name();
        let filename = path.to_str().unwrap();
        let mut raw_path_m = raw_path.to_string();
        let mut dest_path_m = dest_path.to_string();
        raw_path_m.push_str(filename);
        dest_path_m.push_str(filename);
        println!("{}", &raw_path_m);
        process_file(&raw_path_m, &dest_path_m);
    }
}

fn main() {
    let all = true;
    if all {
        process_all_files();
    } else {
        test_single_file();
    }
}
