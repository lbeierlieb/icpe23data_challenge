import json
from functools import reduce
import os


def aggregateBatch(list):
    (sum, count) = reduce(lambda acc, el: (
        acc[0]+el[0]*el[1], acc[1]+el[1]), list, (0, 0))
    return sum/count


def getScale(metric):
    if metric.startswith("s"):
        return 1
    elif metric.startswith("ms"):
        return 1e-3
    elif metric.startswith("us"):
        return 1e-6
    elif metric.startswith("ns"):
        return 1e-9
    else:
        raise ValueError("unhandled metric encountered!")


def processFile(rawFilepath, destinationFilepath):
    with open(rawFilepath, "r") as input, open(destinationFilepath, "w") as output:
        inJson = json.load(input)
        rawData = inJson[0]["primaryMetric"]["rawDataHistogram"]
        metric = inJson[0]["primaryMetric"]["scoreUnit"]
        scale = getScale(metric)
        processed = [[scale*aggregateBatch(batch) for batch in run]
                     for run in rawData]
        json.dump(processed, output, separators=(',', ':'))


def processAllFiles():
    rawPath = "/raw/"
    destPath = "/processed/"

    for file in os.listdir(rawPath):
        print(file)
        processFile(rawPath + file, destPath + file)


def main():
    processAllFiles()


if __name__ == "__main__":
    main()
