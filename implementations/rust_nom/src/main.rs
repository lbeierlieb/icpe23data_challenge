
use std::{fs, io};

use nom::{
    bytes::complete::{tag, take_until, take},
    character::complete::{char, multispace0},
    combinator::map_res,
    number::complete::double,
    sequence::{delimited, pair, separated_pair},
    IResult, error::{Error, ErrorKind},
};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}


fn all<'a>(input: &'a str, output: &mut [f64]) -> IResult<&'a str, ()> {
    let (input, scale) = score_unit(input)?;
    let raw_data_tag = "\"rawDataHistogram\" : ";
    let (input, _) = take_until(raw_data_tag)(input)?;
    let (input, _) = take(raw_data_tag.len())(input)?;

    all_runs(input, output, scale)
}

fn all_runs<'a>(input: &'a str, output: &mut [f64], scale: f64) -> IResult<&'a str, ()> {
    let (input, _) = char('[')(input)?;
    let (input, _) = multispace0(input)?;
    let mut input_loop = input;
    for i in 0..9 {
        let (input, _) = run(input_loop, output, i * 3000, scale)?;
        let (input, _) = char(',')(input)?;
        let (input, _) = multispace0(input)?;
        input_loop = input;
    }
    let (input, _) = run(input_loop, output, 9 * 3000, scale)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(']')(input)?;

    Ok((input, ()))
}

fn run<'a>(input: &'a str, output: &mut [f64], index: usize, scale: f64) -> IResult<&'a str, ()> {
    let (input, _) = char('[')(input)?;
    let (input, _) = multispace0(input)?;
    let mut input_loop = input;
    for i in 0..2999 {
        let (input, measure_mean) = measurements_enclosed(input_loop)?;
        let (input, _) = char(',')(input)?;
        let (input, _) = multispace0(input)?;
        input_loop = input;

        output[index + i] = scale * measure_mean;
    }
    let (input, measure_mean) = measurements_enclosed(input_loop)?;
    output[index + 2999] = scale * measure_mean;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(']')(input)?;

    Ok((input, ()))
}

fn unit_to_f64(unit: &str) -> Result<f64, Error<&str>> {
    match unit {
        "s" => Ok(1.0),
        "ms" => Ok(1e-3),
        "us" => Ok(1e-6),
        "ns" => Ok(1e-9),
        _ => Err(Error { input: unit, code: ErrorKind::Fail})
    }
}

fn score_unit(input: &str) -> IResult<&str, f64> {
    let score_tag = "\"scoreUnit\" : \"";
    let (input, _) = take_until(score_tag)(input)?;
    let (input, _) = take(score_tag.len())(input)?;
    map_res(take_until("/"), unit_to_f64)(input)
}

fn measurements_enclosed(input: &str) -> IResult<&str, f64> {
    let (input, _) = pair(char::<_, Error<_>>('['), multispace0)(input)?;
    let (input, (sum_time, sum_count)) = measurements(input)?;
    let (input, _) = pair(multispace0, char::<_, Error<_>>(']'))(input)?;

    Ok((input, sum_time / sum_count))
}

fn measurements(input: &str) -> IResult<&str, (f64, f64)> {
    let (input, (time, count)) = enclosed_measurement(input)?;
    let res = pair(char::<_, Error<_>>(','), multispace0)(input).ok();
    match res {
        None => Ok((input, (time, count))),
        Some((input, _)) => {
            let (input, (sum_time, sum_count)) = measurements(input)?;
            Ok((input, (sum_time + time * count, sum_count + count)))
        }
    }
}

fn enclosed_measurement(input: &str) -> IResult<&str, (f64, f64)> {
    delimited(tag("[ "), measurement, tag(" ]"))(input)
}

fn measurement(input: &str) -> IResult<&str, (f64, f64)> {
    separated_pair(double, tag(", "), double)(input)
}

fn write(dest_path: &str, data: &[f64]) -> io::Result<()> {
    let mut content = String::new();
    content.push('[');
    for i in 0..10 {
        content.push('[');
        for j in 0..3000 {
            content.push_str(data[i*3000 + j].to_string().as_str());
            content.push(',');
        }
        content.pop();
        content.push_str("],");
    }
    content.pop();
    content.push(']');
    fs::write(dest_path, content)
}

fn main() {
    let filestr = fs::read_to_string("../../jmh/apache__logging-log4j2#org.apache.logging.log4j.perf.jmh.AsyncAppenderLog4j1Benchmark.throughput11Params#.json").unwrap();
    let mut output = [0.0; 30000];
    let mut_slice = &mut output[..];
    all(filestr.as_str(), mut_slice).unwrap();
    let slice = &mut output[..];
    //write("/tmp/jmh/apache__logging-log4j2#org.apache.logging.log4j.perf.jmh.AsyncAppenderLog4j1Benchmark.throughput11Params#.json", slice).unwrap();
}
