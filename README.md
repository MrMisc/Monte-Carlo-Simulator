# Monte Carlo Simulator
 A generalized, LINEAR monte carlo simulator



## Running the program

Like with most of my programmes, type ``py DataEntryPoint.py``, to open the following UI.

![MonteCarloGUI](https://user-images.githubusercontent.com/100022747/214081219-1c7f8ab6-f29f-43bf-bd14-08bf092d14c8.PNG)

As stated before, this is merely for simple linear combinations of random variables, not non-linear configurations. So for example, if you wanted to model the distribution of say the product of a Poisson and a Normal distribution, this would not be an available solution for you. I would encourage in that case, to simply model it and appreciate the process of it yourself in your appropriate language!

Likewise, if you happen to be looking for more than 7 inflow and outflow RVs each, you are out of luck unfortunately.

### Inputs

The available choice of distributions are in the drop down menus. Parameters are to be typed in and separated by commas. So for instance, the Cauchy distribution, iirc, requires 2 parameters (eg 1.0,2.0 as a simple example), while something like the Triangular distribution would require 3 parameters (eg 1.0,3.0,2.0).


![avai](https://user-images.githubusercontent.com/100022747/214092212-1cdb08b4-0c42-475a-8896-4f0ec4a4f913.PNG)


There is an addition functionality that was deemed essential and was hence added to the parameter bar. For all distributions, regardless of number of parameters required for the actual probability distribution, the user is able to indicate a 4th constant float parameter to multiply with the random variable. As an example, if you wanted to model the potential for the occassional large expense related to for instance, medical bills, and had an estimate of each medical incident in your portfolio costing you about 1000 dollars, at a rate of 0.0166, you would choose ``Poisson`` and type out something along the lines - 0.0166,1.0,1.0,1000.0. The 1.0s inside the parameter are of no significance and could have been any other float values, since Poisson only requires one value, the first value in the Parameter array that we have - the rate of incidence.

### Periods

There is a difference between multiplying a RV with a constant say 3, versus adding it 3 times.  So for example, if you wanted to simulate something akin to 3 years or 3 consecutive months. You could, equivalently, simulate 3 Gaussian RVs in Inflows for instance. Alternatively, you could set ``Periods`` to 3.

### Breakeven

Breakeven is an option for the user to allocate a limit value for the programme to compare against. By default, this is set to 0, and any trials that return values BELOW 0, are analyzed and the contributing variables responsible for that trial, are stored and plotted as boxplots/violin plots for further analysis. You can set this to any other static value. Additionally, if you wish the code to record trials that return values ABOVE the breakeven value, type a ``|`` in the breakeven box, AFTER your static value.


## Output

Various graphs are outputted and saved after a successful simulation. Chief amongst them being, density plots of the individual trials at the END of all periods (if periods are allocated to be above 1), histograms of the same thing, values which are shifted, and logged, and boxplots of contributory RV components to fringe values for the first, middle and end periods are saved as figures.

NOTE: these images are all deleted and replaced upon re-simulation. Be sure to save them somewhere else if valuable to you!

The histogram of the resultant vector of trials and the boxplots for each of the 3 periods are shown as a single plot image at the end.


### Results
### 100 000 trials, 50 periods
In the sampled picture above, we simulated a combination of a Normal inflow with mean 15.0 and standard deviation of 15.0, together with 2 sources of outflow, a Poisson RV with rate 0.25 but with a constant multiplier of 170.0, and a Cauchy RV with parameters 1.0 and 2.0. This was executed for 100_000 trials (could imagine them as separate people) for 50 periods. In other words, 50 consecutive periods. These periods are continuations from the predecessing periods - so you could in some instance, imagine this being the portfolios of 100_000 people being simulated over the course of say, 50 years as an example.

The printed output immediately shows you the following

![Sampleoutput](https://user-images.githubusercontent.com/100022747/214087129-2d4220bc-4176-4676-acea-00a91059daec.png)


These and other plots are available in higher resolution in the output as follows

#### Boxplots

![boxplot1](https://user-images.githubusercontent.com/100022747/214087286-1f3f82a8-ed9a-4534-9175-6c363145e04d.png)

Boxplot for the first period is the only one set to non-logarithmic scales, the rest are logged accordingly.

![boxplot25](https://user-images.githubusercontent.com/100022747/214087293-a8d3ffc9-7212-4d6c-a162-59d79b8077fc.png)
![boxplot50](https://user-images.githubusercontent.com/100022747/214087344-694b50e8-0881-4e77-b791-eccbc954f44a.png)

#### Violin plots
![violinplot1](https://user-images.githubusercontent.com/100022747/214087546-9cefae12-bc49-4d5d-9f5c-656fd6ec7fdc.png)

Same goes for violin plots

![violinplot25](https://user-images.githubusercontent.com/100022747/214087569-db982fb5-260d-4e6a-991c-728ff72d229d.png)
![violinplot50](https://user-images.githubusercontent.com/100022747/214087578-609c3a89-d0af-438f-8603-e951f4cd89d5.png)


The final distribution of the trials' results are also shown in 4 formats

#### Logged KDE plot
![Monte_kde_log](https://user-images.githubusercontent.com/100022747/214087716-24f3da90-51b7-4222-9eae-90e7200bec43.png)

#### Histogram plot
![Monte_hist](https://user-images.githubusercontent.com/100022747/214087726-3da7d6a5-0a53-4977-b942-f5e16f39ba6b.png)

#### Logged Histogram plot
![Monte_hist_logged](https://user-images.githubusercontent.com/100022747/214087728-470d850d-b1cf-413f-9c66-f2c95aec29f2.png)

#### KDE plot
![Monte_kde](https://user-images.githubusercontent.com/100022747/214087730-6ef5eda2-1bd8-4dbf-9266-4ac142f01e2f.png)

Hopefully, it is equally as apparent, as understandable, that simulating a bunch of RVs numerous times with numerous trials tends to lead to very spiky and unreadable trial results in histogram and KDE format. This is simply because most Monte Carlo simulations that people would typically be concerned with are fringe situations, and that can only be revealed by repeating an obscene amount of trials and sieving out the exceptional circumstances or the unforeseeable ones.

The unlogged plots also succintly print out the percentage of trials that fell below 0.0 (the default case set here). 


### 100 trials, 50 periods

![Monte_hist](https://user-images.githubusercontent.com/100022747/214094755-bd516fe9-13f1-4aee-a673-8c55d510a56c.png)
![Monte_kde](https://user-images.githubusercontent.com/100022747/214094762-e7b6170d-ab0d-4701-bbb3-cc6ab920bcc8.png)


Alternatively, a more common practice, is often that related professionals often choose to not so intensely simulate large sample sizes to return more observable histograms and their associated plots as the above. However, catching the potential problematic RVs, especially the Cauchy RV become lost in that form of analysis as might be apparent when examining the outputted boxplots.


![boxplot1](https://user-images.githubusercontent.com/100022747/214095200-c430959d-efde-4ebb-97f8-5a91245a1756.png)

If you happen to ask why any negatively contributing RVs are even quotable since the histograms above say that ``0.0000% of trials returned values below 0``, note that this is because in those periods, some trials did in fact end up with negative returns in Period 1, but recovered accordingly in the later periods. Remember that the final histogram and KDEs plotted are only after all the periods have been traversed through. 

As you can see here, all the seeds in this relatively limited simulation, recovered from their negative returns in the subsequent periods.


![boxplot25](https://user-images.githubusercontent.com/100022747/214096237-7c2b086a-987c-4073-8259-921ec722d459.png)
![boxplot50](https://user-images.githubusercontent.com/100022747/214096247-4ef05dcf-b0d4-4a50-80e6-66456e9b25d7.png)

This is a result of the fact that PER PERIOD, the underlying code checks for each trial's state, and if any of the trials are below the set limit (by default again, 0.0), then and only then, are the associated RV contributions logged and plotted. So by Period 25, all of the trials were at a net positive balance (above the fringe csae - being below 0.0)

```rs
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
```

Alternatively, if one found that there were in fact negative contributing factors being presented in the final boxplot, then that could mean that there were indeed cases where the fringe cases were triggered, but they happened as below 0.001% case.


The central message however, is that limiting analyses to low sample sizes potentially excludes the potentially damaging effects of fringe RV effects, especially the Cauchy distribution in this case!

## Additional analysis

In case you are not satisfied with the format of analysis of the contributory RVs for the fringe values, the CSV files for each period are saved in the directory post-simulation for perusal. Note that this is not a record of the actual trial results for each period, rather they are stores of the contributing RVs to fringe values within each period. They are incrementally built on as well from period to period.

This is to say that, if a RV contributed to the fringe values of a trial in period 1, but not in period 23, for instance, it would still be recorded, along with its contribution. This is to ensure that rogue, but large contributions are not carelessly discarded from potential concern.

As in this case, a whopping 50 periods were simulated, it is understandable to the user, hopefully, that it takes a while to render all of this information and store it, especially, in CSV ( something I wrould not recommend someone do, ever, this was a painful experience in data corruption ).


![snapofperiods](https://user-images.githubusercontent.com/100022747/214088986-a69a95cc-a991-4d20-82ba-3ff4303188a1.PNG)

The simulation itself (the RV sample generating and addition and so forth) merely took 3.83117s, but the exporting of values and so forth for further analysis, which had to be done in a higher level language and regrettably, due to my lack of foresight, in CSV format, which is never something I should've done.

## Multi-threading

Multi-threading is by default enabled, and if one wishes to change that, or the number of threads, please be sure to adjust the following segment of code in the main.rs file inside the src directory.

```rs

    trials /= 15;  //Line 70 here, divides the trial numbers by the planned number of threads
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
        for j in 0..15 {// Line 109. Change 15 here to the appropriate number of threads as well.
            let handle = s.spawn(move || {
```



Lines 90 and 109, are the lines you would change accordingly. By default, this programme runs on 15 threads (ideally in parallel , depending on hardware).






