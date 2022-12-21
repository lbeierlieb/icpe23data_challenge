import json
from statistics import mean
from functools import reduce


def processFile(rawFilepath, destinationFilepath):
    #TODO adjust scaling - currently we show us, but actual file is in ms
    with open(rawFilepath, "r") as input, open(destinationFilepath, "w") as output:
        content = input.read()
        inJson = json.loads(content)
        rawData = inJson[0]["primaryMetric"]["rawDataHistogram"]
        processed = list(map(lambda run: list(map(lambda batch: mean(reduce(list.__add__, list(map(lambda x: [x[0]]*x[1], batch)))), run)), rawData))
        outJson = json.dumps(processed)
        output.write(outJson)
        output.flush()


def main():
    rawPath = "jmh/"
    destPath = "/tmp/"
    filename = "apache__arrow#org.apache.arrow.adapter.jdbc.JdbcAdapterBenchmarks.consumeBenchmark#.json"

    processFile(rawPath + filename, destPath + filename)


if __name__ == "__main__":
    main()
