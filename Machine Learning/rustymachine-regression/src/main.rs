#![allow(unused_imports,unused_mut,unused_variables)]
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;
use std::vec::Vec;
use std::error::Error;
use rand::thread_rng;
use rand::seq::SliceRandom;
//use rusty_machine;
//use rusty_machine::linalg::Matrix;
//use rusty_machine::linalg::Vector;

#[derive(Debug,Copy, Clone, PartialEq)]
pub struct BostonHousing {
    crim: f64, zn : f64, indus: f64, chas : f64, nox : f64, rm : f64,
    age : f64, dis : f64, rad : f64, tax : f64, ptratio : f64, black : f64, lstat : f64, medv : f64,
}

impl BostonHousing {
    pub fn new(v : Vec<&str>) -> BostonHousing {
        let f64_formatted : Vec<f64> =  v.iter().map(
            |s| s.parse().unwrap())
            .collect();
        BostonHousing{
            crim : f64_formatted[0], zn : f64_formatted[1], indus : f64_formatted[2],
            chas : f64_formatted[3], nox : f64_formatted[4], rm : f64_formatted[5],
            age : f64_formatted[6], dis: f64_formatted[7], rad : f64_formatted[8],
            tax : f64_formatted[9], ptratio : f64_formatted[10], black : f64_formatted[11],
            lstat : f64_formatted[12], medv: f64_formatted[13]
        }
    }

    pub fn into_feature_vector(&self) -> Vec<f64> {
        vec![self.crim, self.zn, self.indus, self.chas, self.nox, self.rm,
        self.age, self.dis, self.rad,self.tax, self.ptratio, self.black, self.lstat]
    }

    pub fn into_targets(&self) -> f64 {
        self.medv
    }
}


fn get_boston_record(s : String) -> BostonHousing {
    let v : Vec<&str> = s.split_whitespace().collect();
    let b : BostonHousing = BostonHousing::new(v);
    b
}

fn get_boston_records_from_file(
    fl : impl AsRef<Path>) -> Vec<BostonHousing> {
    let file = File::open(fl).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().enumerate()
        .map(|(n, l)| l.expect(
                &format!("Could not parse line number {}", n)
                )).map(|r| get_boston_record(r))
        .collect()
}

pub fn run(){
    let fl = "/home/nicholasnjihia/Documents/boston/Dataset.data";
    let mut data = get_boston_records_from_file(&fl);
    println!("{:?}", data);
    data.shuffle(&mut thread_rng());
    let test_size :f64 = 0.2;
    let test_size :f64 = data.len() as f64 * test_size;
    let test_size = test_size.round() as usize;
    let (test_data, train_data) = data.split_at(test_size);
    let train_size = train_data.len();
    let test_size = test_data.len();
    let boston_x_train: Vec<f64> = train_data.iter()
        .flat_map(|r| r.into_feature_vector())
        .collect();
    let boston_y_train : Vec<f64> = train_data.iter()
        .map(|r| r.into_targets())
        .collect();
    let boston_x_test : Vec<f64> = test_data.iter()
        .flat_map(|r| r.into_feature_vector())
        .collect();
    let boston_y_test : Vec<f64> = test_data.iter().
        map(|r| r.into_targets())
        .collect();
}

fn main() {
    let mut v = Vec::new();
    for i in 1001..=1055{
        v.push(i.to_string());
    }
    println!("v == {:?}",v);
    v.shuffle(&mut thread_rng());
    println!("v after shuffling for first time==");
    println!("{:?}",v);
    v.shuffle(&mut thread_rng());
    println!("v after shuffling for second time==");
    println!("{:?}",v);
    let new_size  = v.len() as f64 * 0.45;
    let new_size = new_size.round() as usize;
    println!("Initial size = {} and new size == {}",v.len(), new_size);
    let (v1,v2) = v.split_at(new_size);
    println!("v1 == {:?}", v1);
    println!();
    println!();
    println!("v2 == {:?}", v2);
    println!();
    println!();
    run();
}
