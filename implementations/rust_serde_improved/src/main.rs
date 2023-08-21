use serde::Deserialize;
use std::{fs, io::Error, io::ErrorKind};

fn get_scale(metric: &str) -> f64 {
    match metric.chars().next().unwrap() {
        's' => 1.0,
        'm' => 1e-3,
        'u' => 1e-6,
        'n' => 1e-9,
        _ => panic!("unknown metric"),
    }
}

fn aggregate_batch(batch: &[Vec<f64>]) -> f64 {
    let (sum, count): (f64, f64) = batch.iter().fold((0.0, 0.0), |(sum, count), measurement| {
        let time = measurement[0];
        let ops = measurement[1];
        (sum + time * ops, count + ops)
    });

    sum / count
}

#[derive(Deserialize)]
struct Benchmark {
    primaryMetric: PrimaryMetric,
}

#[derive(Deserialize)]
struct PrimaryMetric {
    scoreUnit: String,
    rawDataHistogram: Vec<Vec<Vec<Vec<f64>>>>,
}

fn process_file(raw_filepath: &str, destination_filepath: &str) -> Result<(), std::io::Error> {
    let input = fs::read_to_string(raw_filepath)?;

    let in_json: Vec<Benchmark> = serde_json::from_str(&input)?; //.expect("should be fine");
    let primary_metric: &PrimaryMetric = &in_json[0].primaryMetric;
    let scale = get_scale(&primary_metric.scoreUnit);

    let out_json: Vec<_> = primary_metric
        .rawDataHistogram
        .iter()
        .map(|run| -> Vec<_> {
            run.iter()
                .map(|batch| scale * aggregate_batch(batch))
                .collect()
        })
        .collect();

    let output = serde_json::to_string(&out_json)?; //.unwrap();
    fs::write(destination_filepath, &output)?;
    Ok(())
}

fn custom_error(msg: &str) -> Box<dyn FnOnce() -> Error> {
    let msg = msg.to_owned();
    Box::new(|| Error::new(ErrorKind::Other, msg))
}

fn process_all_files() -> Result<(), Error> {
    let raw_path = "/raw/";
    let dest_path = "/processed/";

    for dir_entry in fs::read_dir(raw_path)? {
        let path = dir_entry?.file_name();
        let filename = path
            .to_str()
            .ok_or_else(custom_error("could not convert filepath to str"))?;
        let mut raw_path_m = raw_path.to_string();
        let mut dest_path_m = dest_path.to_string();
        raw_path_m.push_str(filename);
        dest_path_m.push_str(filename);
        println!("{}", &raw_path_m);
        process_file(&raw_path_m, &dest_path_m)?;
    }

    Ok(())
}

fn main() {
    let _ = process_all_files();
}
