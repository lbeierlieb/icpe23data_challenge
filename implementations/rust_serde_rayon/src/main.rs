use rayon::prelude::*;
use serde_json::Value as JsonValue;
use std::fs;

fn get_scale(metric: &str) -> f64 {
    match metric.chars().next().unwrap() {
        's' => 1.0,
        'm' => 1e-3,
        'u' => 1e-6,
        'n' => 1e-9,
        _ => panic!("unknown metric"),
    }
}

fn aggregate_batch(batch: &Vec<JsonValue>) -> f64 {
    let mut sum = 0.0;
    let mut count = 0.0;

    for measurement in batch.iter() {
        let unwrapped = measurement.as_array().unwrap();
        let time = unwrapped[0].as_f64().unwrap();
        let ops = unwrapped[1].as_f64().unwrap();
        sum += time * ops;
        count += ops;
    }

    sum / count
}

fn process_file(raw_filepath: &str, destination_filepath: &str) {
    let input = fs::read_to_string(raw_filepath).unwrap();

    let in_json: JsonValue = serde_json::from_str(&input).expect("should be fine");
    let raw_data = in_json[0]["primaryMetric"]["rawDataHistogram"]
        .as_array()
        .unwrap();
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

fn process_dir_entry(dir_entry: fs::DirEntry, raw_folder: &str, dest_folder: &str) {
    let path = dir_entry.file_name();
    let filename = path.to_str().unwrap();
    let mut raw_path_m = raw_folder.to_string();
    let mut dest_path_m = dest_folder.to_string();
    raw_path_m.push_str(filename);
    dest_path_m.push_str(filename);
    println!("{}", &raw_path_m);
    process_file(&raw_path_m, &dest_path_m);
}

fn process_all_files() {
    let raw_path = "/raw/";
    let dest_path = "/processed/";

    fs::read_dir(raw_path)
        .unwrap()
        .into_iter()
        .par_bridge()
        .for_each(|dir_entry| process_dir_entry(dir_entry.unwrap(), raw_path, dest_path));
}

fn main() {
    process_all_files();
}
