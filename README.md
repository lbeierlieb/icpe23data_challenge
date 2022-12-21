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
