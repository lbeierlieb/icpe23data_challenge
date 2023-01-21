# ICPEDataChallenge2023

[Link to Website](https://icpe2023.spec.org/tracks-and-submissions/data-challenge-track/)

The provided raw data consists of 600 JMH benchmark results in JSON format.
Each benchmark consists of 10 runs, which consists of 3000 batches of around
10-100 measurements (~65GB in total, 14 of the files are invalid).  

The provided preprocessed data differs from that in the regard that the
~10-100 measurements of each batch are aggregated to their mean value.
The resulting timeseries are also saved as JSON files - per file 10 runs with
3000 aggregated values.
The goal of the data challenge paper is to compare different implementations
of this processing step. We measure the execution times required by different
programming languages using different parsing libraries and strategies.

## Requirements

- ~75GB of storage
- `bash` to execute the scripts
- `docker` to build and run the containers with different implementations

## Usage

- run `./get_raw_data` to retrieve the raw data from the web.
It is stored as `jmh.tar.gz` (4.5GB), and extract to `jmh/` (65GB).
The 14 invalid files (noted in `invalid_raw_files`) are deleted.
The files designated for benchmarking (noted in `benchfiles`) are copied to the folder `jmh_bench/`
- run `./get_processed_data` to retrieve the processed data from Github to `icpe-data-challenge-jmh/` (0.5GB).
This is not necessary for data processing, but can be used together with the Python application in `result_verifier/` to check that the generated JSON files are correct.
- `implementations/` contains the code for the different programming languages.
- `builds/` contains the Dockerfiles to build an image for every implementation variant.
The folder names in `builds/` are used as "buildname"s in later mentioned scripts.
- run `./build <buildname>` to build the container image <buildname>
- run `./build_all` to build all images within `builds/`
- Make sure you build the image for a buildname before you try to run them
- The `run` bash script is used to execute built images and measure container execution time.
By default, the script mounts `/tmp/processed/` to the container, which will then store its processed files there.
By default, the script limits containers to a maximum RAM usage of 26GB.
You can change the destination path and the container memory limit at the top of the `run` script.
If you assign less then 26GB of memory, the parallelized rust_serde_rayon builds might reach the limit and terminate the container early.
- run `./run --bench <buildname>` to let buildname process all the files in `jmh_bench/`.
The execution time together with <buildname> is stored in `bench_log.csv`. Run `./init_bench_log` to clear the log and provide a CSV header line.
- run `./run --all <buildname>` to let buildname process all the 586 files in `jmh`.
The execution time together with the buildname is stored in `all_log.csv`. Run `./init_all_log` to clear the log and provide a CSV header line.
- `icpe23dc_measurements/` contains the datasets presented in the ICPE23 data challenge paper.
