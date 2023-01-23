use rand::distributions::{Distribution, Standard};
use rand::thread_rng;
use statrs::distribution::{
    Beta, Cauchy, Chi, ChiSquared, Dirac, Dirichlet, Erlang, Exp, FisherSnedecor, Gamma, Geometric,
    Hypergeometric, InverseGamma, Laplace, LogNormal, Multinomial, MultivariateNormal,
    NegativeBinomial, Normal, Pareto, Poisson, StudentsT, Uniform, Weibull,Triangular,
};
use std::thread;
extern crate serde;
extern crate serde_json;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::json;

// use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::time::{Duration, Instant};
use std::{fs, io, process};
use std::error::Error;

use csv;
// #[derive(Deserialize)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    inflows: Vec<RandomVariable>,
    outflows: Vec<RandomVariable>,
    period:u64,
    stakeholder:f64,
    morse:String,
    trials: u64,
}
// #[derive(Deserialize)]
#[derive(Serialize, Deserialize, Debug)]
struct RandomVariable {
    distribution: String,
    parameters: [f64; 4],
}

struct Var{
    inflow_or_outflow:String,
    distribution:String,
    value:f64,
}

fn main() {
    // let mut rng = thread_rng();
    // let v: Vec<f64> = Gamma::new(1.0,3.0).unwrap().sample_iter(&mut rng).take(160).collect();
    // let n: Vec<f64> = Normal::new(1.0,3.0).unwrap().sample_iter(&mut rng).take(160).collect();
    
    // dbg!(v);
    
    // dbg!(n);

    
    // let mut rng = thread_rng();
    // let v: Vec<f64> = Weibull::new(1.0,3.0).unwrap().sample_iter(&mut rng).take(160).collect();
    // dbg!(v);

    let mut file = File::open("./output.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let data: Params = serde_json::from_str(&buff).unwrap();

    
    let mut trials: usize = data.trials.try_into().unwrap();
    trials /= 15;
    let mut distributions: Vec<String> = Vec::new();
    let mut parameter_arr: Vec<[f64; 4]> = Vec::new();
    let mut neg_distributions: Vec<String> = Vec::new();
    let mut neg_parameter_arr: Vec<[f64; 4]> = Vec::new();
    for element in data.inflows {
        distributions.push(element.distribution);
        parameter_arr.push(element.parameters);
    }
    for element2 in data.outflows{
        neg_distributions.push(element2.distribution);
        neg_parameter_arr.push(element2.parameters);
    }
    let distributions_th: &[String] = &distributions;
    let parameter_arr_th: &[[f64; 4]] = &parameter_arr;
    let neg_distributions_th: &[String] = &neg_distributions;
    let neg_parameter_arr_th: &[[f64; 4]] = &neg_parameter_arr;    
    let period:u64 = data.period;
    let stakeholder:f64 = data.stakeholder;
    let switch: bool = match &data.morse as &str{
        "true" => true,
        "false" => false,
        _ => true,
    };
    // let mut outputflow: Vec<f64> = Vec::new();    
    let duration = Instant::now();
    //Prior to starting the multithreaded process, to avoid duplicate headers in each csv file, we write the period files pre-emptively
    for _iter0 in 0..period{
        let filestring:String = String::from("./Period") +&(_iter0+1).to_string() + ".csv";
        let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&filestring)
        .unwrap();                    
        let mut wtr = csv::Writer::from_writer(file);
        wtr.write_record(&["Source Type", "Distribution", "Amount"]);
    }
    thread::scope(|s| {
        for j in 0..15 {
            let handle = s.spawn(move || {
                let mut flow: Vec<f64> = Vec::new();
                let mut vector_ofvars_pertrial: Vec<Vec<Var>> = Vec::new();
                for _i in 0..trials {
                    flow.push(0.0);
                    vector_ofvars_pertrial.push(Vec::new());
                }                
                // let mut negative_record: Vec<(&str,f64)> = Vec::new();
                // for _k in 0..trials{
                //     negative_record.push(("Nil", 0.0));
                // }
                // let mut negative_input_record: Vec<(&str,f64)> = Vec::new();
                // for _k in 0..trials{
                //     negative_input_record.push(("Nil", 0.0));
                // }                                
                for _iter in 0..period{
                    //Uncomment below if you want the negative records to be only specific for contributing sources in the latest period, instead of cumulatively from period 1. Be sure to comment out vector_ofvars_pertrial above too to switch
                    // let mut vector_ofvars_pertrial: Vec<Vec<Var>> = Vec::new();
                    // for thing in 0..trials{
                    //     vector_ofvars_pertrial.push(Vec::new());
                    // }                    
                    let distributions_thr: &[String] = distributions_th.clone();
                    let parameter_arr_thr: &[[f64; 4]] = parameter_arr_th.clone();                
                    for no in 0..distributions_thr.len() {
                        let mut rng = thread_rng();
                        if distributions_thr[no] == "Dirac" {
                            let v: &Vec<f64> = &Dirac::new(*&parameter_arr_thr[no][0])
                                .unwrap()
                                .sample_iter(&mut rng)
                                .take(trials as usize)
                                .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    // if v[i]<0.0{negative_record[i] = (&distributions_thr[no],v[i]);}
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){
                                        // println!("So we have..{}",(no as u8+1).to_string()+&distributions_thr[no].clone());
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});
                                    }
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        } else if distributions_thr[no] == "Normal" {
                            let v: &Vec<f64> =
                                &Normal::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        } else if distributions_thr[no] == "Gamma" {
                            let v: &Vec<f64> =
                                &Gamma::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }
                        else if distributions_thr[no] == "Beta" {
                            let v: &Vec<f64> =
                                &Beta::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }                    
                        else if distributions_thr[no] == "Cauchy" {
                            let v: &Vec<f64> =
                                &Cauchy::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }
                        else if distributions_thr[no] == "Chi" {
                            let v: &Vec<f64> =
                                &Chi::new(*&parameter_arr_thr[no][0])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }                      
                        else if distributions_thr[no] == "ChiSquared" {
                            let v: &Vec<f64> =
                                &ChiSquared::new(*&parameter_arr_thr[no][0])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }
                        else if distributions_thr[no] == "Triangular" {
                            let v: &Vec<f64> =
                                &Triangular::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1],*&parameter_arr_thr[no][2])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }
                        // else if distributions_thr[no] == "Dirichlet" {
                        //     let v: &Vec<f64> =
                        //         &Dirichlet::new(vec![*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1], *&parameter_arr_thr[no][2]])
                        //             .unwrap()
                        //             .sample_iter(&mut rng)
                        //             .take(trials as usize)
                        //             .collect();
                        //     if parameter_arr_thr[no][3] == 1.0 {
                        //         for i in 0..trials {
                        //             flow[i] += v[i];
                        //         }
                        //     } else {
                        //         for i in 0..trials {
                        //             flow[i] += parameter_arr_thr[no][3] * v[i];
                        //         }
                        //     }
                        // }
                        else if distributions_thr[no] == "Erlang" {
                            let v: &Vec<f64> =
                                &Erlang::new(*&parameter_arr_thr[no][0].max(std::u64::MIN as f64).min(std::u64::MAX as f64).round() as u64, *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }
                        else if distributions_thr[no] == "Exp" {
                            let v: &Vec<f64> =
                                &Exp::new(*&parameter_arr_thr[no][0])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }        
                        else if distributions_thr[no] == "FisherSnedecor" {
                            let v: &Vec<f64> =
                                &FisherSnedecor::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }          
                        else if distributions_thr[no] == "Geometric" {
                            let v: &Vec<f64> =
                                &Geometric::new(*&parameter_arr_thr[no][0])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }    
                        else if distributions_thr[no] == "Hypergeometric" {
                            let v: &Vec<f64> =
                                &Hypergeometric::new(*&parameter_arr_thr[no][0].max(std::u64::MIN as f64).min(std::u64::MAX as f64).round() as u64, *&parameter_arr_thr[no][1].max(std::u64::MIN as f64).min(std::u64::MAX as f64).round() as u64, *&parameter_arr_thr[no][0].max(std::u64::MIN as f64).min(std::u64::MAX as f64).round() as u64)
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }
                        else if distributions_thr[no] == "InverseGamma" {
                            let v: &Vec<f64> =
                                &InverseGamma::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }
                        else if distributions_thr[no] == "Laplace" {
                            let v: &Vec<f64> =
                                &Laplace::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }  
                        else if distributions_thr[no] == "LogNormal" {
                            let v: &Vec<f64> =
                                &LogNormal::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }                    
                        else if distributions_thr[no] == "NegativeBinomial" {
                            let mut v: &Vec<u64> =
                                &NegativeBinomial::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i] as f64;
                                    // vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i] as f64});;
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * (v[i] as f64);
                                    // vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i] as f64});;
                                }
                            }
                        }               
                        else if distributions_thr[no] == "Pareto" {
                            let v: &Vec<f64> =
                                &Pareto::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }   
                        else if distributions_thr[no] == "Poisson" {
                            let v: &Vec<f64> =
                                &Poisson::new(*&parameter_arr_thr[no][0])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }
                        else if distributions_thr[no] == "StudentsT" {
                            let v: &Vec<f64> =
                                &StudentsT::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1], *&parameter_arr_thr[no][2])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }
                        else if distributions_thr[no] == "Uniform" {
                            let mut set: [u8;2] = [0,0];
                            if &parameter_arr_thr[no][0]<&parameter_arr_thr[no][1]{
                                set[1]+=1;
                            }else{
                                set[0]+=1;
                            }
                            let v: &Vec<f64> =
                                &Uniform::new(*&parameter_arr_thr[no][set[0] as usize], *&parameter_arr_thr[no][set[1] as usize])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }
                        else if distributions_thr[no] == "Weibull" {
                            let v: &Vec<f64> =
                                &Weibull::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] += v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:v[i]});}
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] += parameter_arr_thr[no][3] * v[i];
                                    if (v[i]<0.0 && switch) || (v[i]>0.0 && !switch){vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("inflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:parameter_arr_thr[no][3]*v[i]});}
                                }
                            }
                        }                                               
                                                                                   
                                                                                                                              
                    }
                    let distributions_thr: &[String] = neg_distributions_th.clone();
                    let parameter_arr_thr: &[[f64; 4]] = neg_parameter_arr_th.clone();                 
                    for no in 0..distributions_thr.len() {
                        let mut rng = thread_rng();
                        if distributions_thr[no] == "Dirac" {
                            let v: &Vec<f64> = &Dirac::new(*&parameter_arr_thr[no][0])
                                .unwrap()
                                .sample_iter(&mut rng)
                                .take(trials as usize)
                                .collect();                            
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];                                                                    
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        // println!("thing is {}", (no as u8+1).to_string()+&distributions_thr[no].clone());
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                    
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];                                                                        
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        // println!("thing is {}", (no as u8+1).to_string()+&distributions_thr[no].clone());
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3]*v[i]});
                                    } 
                                }
                            }
                        } else if distributions_thr[no] == "Normal" {
                            let v: &Vec<f64> =
                                &Normal::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3]*v[i]});
                                    }                                       
                                }
                            }
                        } else if distributions_thr[no] == "Gamma" {
                            let v: &Vec<f64> =
                                &Gamma::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3]*v[i]});
                                    }                                       
                                }
                            }
                        }
                        else if distributions_thr[no] == "Beta" {
                            let v: &Vec<f64> =
                                &Beta::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3]*v[i]});
                                    }                                       
                                }
                            }
                        }                    
                        else if distributions_thr[no] == "Cauchy" {
                            let v: &Vec<f64> =
                                &Cauchy::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3]*v[i]});
                                    }                                       
                                }
                            }
                        }
                        else if distributions_thr[no] == "Triangular" {
                            let v: &Vec<f64> =
                                &Triangular::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1],*&parameter_arr_thr[no][2])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3]*v[i]});
                                    }                                       
                                }
                            }
                        }                        
                        else if distributions_thr[no] == "Chi" {
                            let v: &Vec<f64> =
                                &Chi::new(*&parameter_arr_thr[no][0])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3]*v[i]});
                                    }                                       
                                }
                            }
                        }                      
                        else if distributions_thr[no] == "ChiSquared" {
                            let v: &Vec<f64> =
                                &ChiSquared::new(*&parameter_arr_thr[no][0])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3]*v[i]});
                                    }                                       
                                }
                            }
                        }
                        // else if distributions_thr[no] == "Dirichlet" {
                        //     let v: &Vec<f64> =
                        //         &Dirichlet::new(vec![*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1], *&parameter_arr_thr[no][2]])
                        //             .unwrap()
                        //             .sample_iter(&mut rng)
                        //             .take(trials as usize)
                        //             .collect();
                        //     if parameter_arr_thr[no][3] == 1.0 {
                        //         for i in 0..trials {
                        //             flow[i] += v[i];
                        //         }
                        //     } else {
                        //         for i in 0..trials {
                        //             flow[i] += parameter_arr_thr[no][3] * v[i];
                        //         }
                        //     }
                        // }
                        else if distributions_thr[no] == "Erlang" {
                            let v: &Vec<f64> =
                                &Erlang::new(*&parameter_arr_thr[no][0].max(std::u64::MIN as f64).min(std::u64::MAX as f64).round() as u64, *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3]*v[i]});
                                    }                                       
                                }
                            }
                        }
                        else if distributions_thr[no] == "Exp" {
                            let v: &Vec<f64> =
                                &Exp::new(*&parameter_arr_thr[no][0])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3]*v[i]});
                                    }                                       
                                }
                            }
                        }        
                        else if distributions_thr[no] == "FisherSnedecor" {
                            let v: &Vec<f64> =
                                &FisherSnedecor::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3]*v[i]});
                                    }                                       
                                }
                            }
                        }          
                        else if distributions_thr[no] == "Geometric" {
                            let v: &Vec<f64> =
                                &Geometric::new(*&parameter_arr_thr[no][0])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3]*v[i]});
                                    }                                       
                                }
                            }
                        }    
                        else if distributions_thr[no] == "Hypergeometric" {
                            let v: &Vec<f64> =
                                &Hypergeometric::new(*&parameter_arr_thr[no][0].max(std::u64::MIN as f64).min(std::u64::MAX as f64).round() as u64, *&parameter_arr_thr[no][1].max(std::u64::MIN as f64).min(std::u64::MAX as f64).round() as u64, *&parameter_arr_thr[no][0].max(std::u64::MIN as f64).min(std::u64::MAX as f64).round() as u64)
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3]*v[i]});
                                    }                                       
                                }
                            }
                        }
                        else if distributions_thr[no] == "InverseGamma" {
                            let v: &Vec<f64> =
                                &InverseGamma::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3]*v[i]});
                                    }                                       
                                }
                            }
                        }
                        else if distributions_thr[no] == "Laplace" {
                            let v: &Vec<f64> =
                                &Laplace::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3]*v[i]});
                                    }                                       
                                }
                            }
                        }  
                        else if distributions_thr[no] == "LogNormal" {
                            let v: &Vec<f64> =
                                &LogNormal::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3]*v[i]});
                                    }                                       
                                }
                            }
                        }                    
                        else if distributions_thr[no] == "NegativeBinomial" {
                            let mut v: &Vec<u64> =
                                &NegativeBinomial::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i] as f64;
                                    if (v[i]>0 && switch) || (v[i]<0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-(v[i] as f64)});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * (v[i] as f64);
                                    if (v[i]>0 && switch) || (v[i]<0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3] * (v[i] as f64)});
                                    }                                       
                                }
                            }
                        }               
                        else if distributions_thr[no] == "Pareto" {
                            let v: &Vec<f64> =
                                &Pareto::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3] * v[i]});
                                    }                                       
                                }
                            }
                        }   
                        else if distributions_thr[no] == "Poisson" {
                            let v: &Vec<f64> =
                                &Poisson::new(*&parameter_arr_thr[no][0])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3] * v[i]});
                                    }                                       
                                }
                            }
                        }
                        else if distributions_thr[no] == "StudentsT" {
                            let v: &Vec<f64> =
                                &StudentsT::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1], *&parameter_arr_thr[no][2])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3] * v[i]});
                                    }                                       
                                }
                            }
                        }
                        else if distributions_thr[no] == "Uniform" {
                            let mut set: [u8;2] = [0,0];
                            if &parameter_arr_thr[no][0]<&parameter_arr_thr[no][1]{
                                set[1]+=1;
                            }else{
                                set[0]+=1;
                            }
                            let v: &Vec<f64> =
                                &Uniform::new(*&parameter_arr_thr[no][set[0] as usize], *&parameter_arr_thr[no][set[1] as usize])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3] * v[i]});
                                    }                                       
                                }
                            }
                        }
                        else if distributions_thr[no] == "Weibull" {
                            let v: &Vec<f64> =
                                &Weibull::new(*&parameter_arr_thr[no][0], *&parameter_arr_thr[no][1])
                                    .unwrap()
                                    .sample_iter(&mut rng)
                                    .take(trials as usize)
                                    .collect();
                            if parameter_arr_thr[no][3] == 1.0 {
                                for i in 0..trials {
                                    flow[i] -= v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-v[i]});
                                    }                                       
                                }
                            } else {
                                for i in 0..trials {
                                    flow[i] -= parameter_arr_thr[no][3] * v[i];
                                    if (v[i]>0.0 && switch) || (v[i]<0.0 && !switch){
                                        vector_ofvars_pertrial[i].push(Var{inflow_or_outflow:String::from("outflow"),distribution:(no as u8+1).to_string()+&distributions_thr[no].clone(),value:-parameter_arr_thr[no][3] * v[i]});
                                    }                                       
                                }
                            }
                        }                                               
                                                                                   
                                                                                                                              
                    }
                    //Open CSV
                    let filestring:String = String::from("./Period") +&(_iter+1).to_string() + ".csv";
                    let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(&filestring)
                    .unwrap();                    
                    let mut wtr = csv::Writer::from_writer(file);
                    // wtr.write_record(&["Source Type", "Distribution", "Amount"]);
                    //Negative records
                    for i in 0..trials{
                        if flow[i]<stakeholder{
                            let mut _ve:&mut Vec<Var> = &mut vector_ofvars_pertrial[i];
                            _ve.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
                            
                            for y in _ve{
                                // println!("{}",y.distribution.clone());
                                wtr.write_record(&[y.inflow_or_outflow.clone(), y.distribution.clone(), y.value.to_string()]);
                            }
                        }
                    }

                }
            
                // let outputflow: &[f64] = &(flow.clone());
                // outputflow                
                for i in flow.iter(){
                    println!("{}",i);
                }

            });
            
        }});
    let timethink = duration.elapsed();
    let duration1 = timethink.as_secs();
    let extra_time_info = timethink.subsec_nanos();
    let dur = json!({
        "Time":duration1,
        "Extended Time Details":extra_time_info
    });

    fs::remove_file("duration.json").expect("File delete failed!");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("duration.json")
        .unwrap();
    write!(file, "{}", dur).unwrap();
}
