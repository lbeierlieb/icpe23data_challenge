# ICPEDataChallenge2023_Lukas

[Link to Website](https://icpe2023.spec.org/tracks-and-submissions/data-challenge-track/)

The provided raw data consists of 600 JMH benchmark results in JSON format.
Each benchmark consists of 10 runs, which consists of 3000 batches of around
10-100 measurements (~65GB in total, 14 of the files are invalid).  

The provided preprocessed data differs from that in the regard that the
10-100 measurements of each batch are aggregated to their mean value.
The resulting timeseries are also saved as JSON files - per file 10 runs with
3000 aggregated values.
The goal of the data challenge paper is to compare different implementations
of this processing step. We measure the execution times required by different
programming languages using different parsing libraries and strategies.

Measured results on Lukas laptop to get an impression:
| Impl.      | Time        | Result                                                |
|------------|-------------|--------------------------------------|
| Python     | 00:17:36.00 | 967.25s user, 78.42s system          |
| R rjson    | 03:00:56.25 | 10679.68s user 36.22s system         |
| Rust serde | 00:04:44.21 | 247.45s user 23.61s system           |
