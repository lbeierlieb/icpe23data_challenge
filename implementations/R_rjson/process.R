library(rjson)
library(purrr)

aggregate_batch <- function(batch) {
    sum_and_count <- Reduce(
        function(acc, el) c(acc[1] + el[1] * el[2], acc[2] + el[2]),
        batch, c(0, 0)
    )

    sum_and_count[1] / sum_and_count[2]
}

get_scaler <- function(metric) {
    if (startsWith(metric, "s")) {
        scale <- 1
    } else if (startsWith(metric, "ms")) {
        scale <- 1e-3
    } else if (startsWith(metric, "us")) {
        scale <- 1e-6
    } else if (startsWith(metric, "ns")) {
        scale <- 1e-9
    } else {
        stop("unhandled metric encountered")
    }

    function(val) {
        val * scale
    }
}

process_file <- function(raw_filepath, destination_filepath) {
    in_json <- fromJSON(file = raw_filepath)
    raw_data <- in_json[[1]]$primaryMetric$rawDataHistogram
    metric <- in_json[[1]]$primaryMetric$scoreUnit
    scaler <- get_scaler(metric)
    processed <- lapply(raw_data, function(run) lapply(run, function(batch) scaler(aggregate_batch(batch))))
    out_json <- toJSON(processed)
    write(out_json, destination_filepath)
}

raw_path <- "jmh/"
dest_path <- "/tmp/jmh/"
dir.create(dest_path, showWarnings = FALSE, recursive = TRUE)

# filename <- "apache__arrow#org.apache.arrow.adapter.jdbc.JdbcAdapterBenchmarks.consumeBenchmark#.json" # 21 MBpy
# filename <- "apache__logging-log4j2#org.apache.logging.log4j.perf.jmh.AsyncAppenderLog4j1Benchmark.throughput11Params#.json"  # 1.9GB
 filename <- "apache__logging-log4j2#org.apache.logging.log4j.perf.jmh.AsyncLoggersBenchmark.throughput9Params#.json" # 698 MB
# filename <- "RoaringBitmap__RoaringBitmap#org.roaringbitmap.aggregation.andnot.worstcase.Roaring64BitmapBenchmark.inplace_andNot#.json"  # 24 MB
# filename <- "JCTools__JCTools#org.jctools.jmh.baseline.SingleThreadedPoll.poll#qType=MpscArrayQueue.json" # 9 MB

process_file(paste(raw_path, filename, sep = ""), paste(dest_path, filename, sep = ""))
