library(rjson)
library(purrr)

aggregate_batch <- function(batch) {
    sum_and_count <- Reduce(
        function(acc, el) c(acc[1] + el[1] * el[2], acc[2] + el[2]),
        batch, c(0, 0)
    )

    sum_and_count[1] / sum_and_count[2]
}

get_scale <- function(metric) {
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

    scale
}

process_file <- function(raw_filepath, destination_filepath) {
    in_json <- fromJSON(file = raw_filepath)
    raw_data <- in_json[[1]]$primaryMetric$rawDataHistogram
    metric <- in_json[[1]]$primaryMetric$scoreUnit
    scale <- get_scale(metric)
    processed <- lapply(raw_data, function(run) lapply(run, function(batch) scale*aggregate_batch(batch)))
    out_json <- toJSON(processed)
    write(out_json, destination_filepath)
}

processAllFiles <- function() {
    raw_path <- "/raw/"
    dest_path <- "/processed/"
    dir.create(dest_path, showWarnings = FALSE, recursive = TRUE)

    for (filename in list.files(raw_path)) {
        print(filename)
        process_file(paste(raw_path, filename, sep = ""), paste(dest_path, filename, sep = ""))
    }
}

processAllFiles()
