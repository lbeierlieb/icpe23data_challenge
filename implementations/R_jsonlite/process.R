library(jsonlite)

aggregate_batch <- function(batch) {
    sum <- 0
    count <- 0
    for (i in 1:(length(batch) / 2)) {
        sum <- sum + batch[i, 1] * batch[i, 2]
        count <- count + batch[i, 2]
    }
    sum / count
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
    in_json <- fromJSON(raw_filepath)
    raw_data <- in_json[["primaryMetric"]][["rawDataHistogram"]][[1]]
    metric <- in_json[["primaryMetric"]][["scoreUnit"]]
    scaler <- get_scaler(metric)
    processed <- lapply(raw_data, function(run) lapply(run, function(batch) scaler(aggregate_batch(batch))))
    out_json <- toJSON(processed, auto_unbox = TRUE)
    write(out_json, destination_filepath)
}

testSingleFile <- function() {
    raw_path <- "jmh/"
    dest_path <- "/tmp/jmh/"
    dir.create(dest_path, showWarnings = FALSE, recursive = TRUE)

    # filename <- "apache__arrow#org.apache.arrow.adapter.jdbc.JdbcAdapterBenchmarks.consumeBenchmark#.json" # 21 MBpy
    # filename <- "apache__logging-log4j2#org.apache.logging.log4j.perf.jmh.AsyncAppenderLog4j1Benchmark.throughput11Params#.json"  # 1.9GB
    filename <- "apache__logging-log4j2#org.apache.logging.log4j.perf.jmh.AsyncLoggersBenchmark.throughput9Params#.json" # 698 MB
    # filename <- "RoaringBitmap__RoaringBitmap#org.roaringbitmap.aggregation.andnot.worstcase.Roaring64BitmapBenchmark.inplace_andNot#.json"  # 24 MB
    # filename <- "JCTools__JCTools#org.jctools.jmh.baseline.SingleThreadedPoll.poll#qType=MpscArrayQueue.json" # 9 MB

    process_file(paste(raw_path, filename, sep = ""), paste(dest_path, filename, sep = ""))
}

processAllFiles <- function() {
    raw_path <- "jmh/"
    dest_path <- "/tmp/jmh/"
    dir.create(dest_path, showWarnings = FALSE, recursive = TRUE)

    for (filename in list.files(raw_path)) {
        print(filename)
        process_file(paste(raw_path, filename, sep = ""), paste(dest_path, filename, sep = ""))
    }
}

testSingleFile()
# processAllFiles()
