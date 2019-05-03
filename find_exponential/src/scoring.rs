use ndarray::{Array1, Array2};
use ndarray_stats::{CorrelationExt, SummaryStatisticsExt};
use itertools::Itertools;

#[allow(dead_code)]
pub fn pearson_correlation(array: &Vec<[f64; 2]>) -> (f64, bool) {
    let array = Array2::from(array.clone());
    let array = array.t();
    let corr = array.pearson_correlation()[[1, 0]];
    (corr, corr < super::ACCEPTANCE_CORRELATION)
}

#[allow(dead_code)]
pub fn slope_stddev(array: &Vec<[f64; 2]>) -> (f64, bool) {
    let mut slopes = Vec::with_capacity(super::SAMPLE_SIZE * (super::SAMPLE_SIZE-1) / 2);
    slopes.extend(
        (0..super::SAMPLE_SIZE).tuple_combinations()
            .map(|(a, b)| {
                let x1 = array[a][0];
                let y1 = array[a][1];
                let x2 = array[b][0];
                let y2 = array[b][1];
                let dx = x2 - x1;
                let dy = y2 - y1;
                let slope = dy / dx;
                slope
            }).filter(|&slope| slope > 0.0)
    );

    let slopes = Array1::from(slopes);
    if super::DEBUG_LEVEL >= 2 {
        println!("slopes: {}", slopes);
    }

    if slopes.is_empty() {
        // Values were too small, we just assume linear behaviour.
        // Otherwise we should see larger values.
        return (0.0, false);
    }

    let stddev = slopes.central_moment(2).unwrap().sqrt();

    (stddev, stddev > super::ACCEPTANCE_STDDEV)
}