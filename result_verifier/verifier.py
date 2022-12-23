import json
import os


def areValuesCloseEnough(val1, val2, maxDiff):
    return abs((val1-val2)/val1) < maxDiff


def checkFile(origFilepath, futFilepath):  # fut == file under test
    with open(origFilepath, "r") as origFile, open(futFilepath, "r") as futFile:
        origJson = json.load(origFile)
        futJson = json.load(futFile)
        for runIndex in range(0, 10):
            for batchIndex in range(0, 3000):
                if not areValuesCloseEnough(origJson[runIndex][batchIndex], futJson[runIndex][batchIndex], 0.0001):
                    return False
        return True


def testSingleFile():
    origPath = "icpe-data-challenge-jmh/timeseries/"
    futPath = "/tmp/jmh/"

    # filename = "apache__arrow#org.apache.arrow.adapter.jdbc.JdbcAdapterBenchmarks.consumeBenchmark#.json" # 21 MB
    # filename = "apache__logging-log4j2#org.apache.logging.log4j.perf.jmh.AsyncAppenderLog4j1Benchmark.throughput11Params#.json"  # 1.9GB
    filename = "apache__logging-log4j2#org.apache.logging.log4j.perf.jmh.AsyncLoggersBenchmark.throughput9Params#.json"  # 698 MB
    # filename = "RoaringBitmap__RoaringBitmap#org.roaringbitmap.aggregation.andnot.worstcase.Roaring64BitmapBenchmark.inplace_andNot#.json"  # 24 MB

    return checkFile(origPath + filename, futPath + filename)


def processAllFiles():
    origPath = "icpe-data-challenge-jmh/timeseries/"
    futPath = "/tmp/jmh/"

    for file in os.listdir(rawPath):
        print(file)
        if not checkFile(rawPath + file, destPath + file):
            return False


def main():
    res = testSingleFile()
    # res = processAllFiles()
    print(res)
    if res:
        return 0
    else:
        return 1


if __name__ == "__main__":
    main()
