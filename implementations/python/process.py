import json
from functools import reduce
import os


def aggregateBatch(list):
    (sum, count) = reduce(lambda acc, el: (
        acc[0]+el[0]*el[1], acc[1]+el[1]), list, (0, 0))
    return sum/count


def getScaler(metric):
    if metric.startswith("s"):
        scale = 1
    elif metric.startswith("ms"):
        scale = 1e-3
    elif metric.startswith("us"):
        scale = 1e-6
    elif metric.startswith("ns"):
        scale = 1e-9
    else:
        raise ValueError("unhandled metric encountered!")
    return lambda val: scale*val


def processFile(rawFilepath, destinationFilepath):
    with open(rawFilepath, "r") as input, open(destinationFilepath, "w") as output:
        inJson = json.load(input)
        rawData = inJson[0]["primaryMetric"]["rawDataHistogram"]
        metric = inJson[0]["primaryMetric"]["scoreUnit"]
        scaler = getScaler(metric)
        processed = [[scaler(aggregateBatch(batch)) for batch in run]
                     for run in rawData]
        json.dump(processed, output)


def testSingleFile():
    rawPath = "jmh/"
    destPath = "/tmp/jmh/"
    os.makedirs(destPath, exist_ok=True)

    # filename = "apache__arrow#org.apache.arrow.adapter.jdbc.JdbcAdapterBenchmarks.consumeBenchmark#.json" # 21 MB
    # filename = "apache__logging-log4j2#org.apache.logging.log4j.perf.jmh.AsyncAppenderLog4j1Benchmark.throughput11Params#.json"  # 1.9GB
    filename = "apache__logging-log4j2#org.apache.logging.log4j.perf.jmh.AsyncLoggersBenchmark.throughput9Params#.json"  # 698 MB
    # filename = "RoaringBitmap__RoaringBitmap#org.roaringbitmap.aggregation.andnot.worstcase.Roaring64BitmapBenchmark.inplace_andNot#.json"  # 24 MB

    processFile(rawPath + filename, destPath + filename)


def processAllFiles():
    rawPath = "jmh/"
    destPath = "/tmp/jmh/"
    os.makedirs(destPath, exist_ok=True)

    for file in os.listdir(rawPath):
        print(file)
        processFile(rawPath + file, destPath + file)


def main():
    testSingleFile()
    # processAllFiles()


if __name__ == "__main__":
    main()
