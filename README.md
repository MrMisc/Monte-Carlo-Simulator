# Monte Carlo Simulator
 A generalized, LINEAR monte carlo simulator



## Running the program

Like with most of my programmes, a python file activates the application pipeline ``DataEntryPoint.py``, to open the following UI.

![MonteCarloGUI](https://user-images.githubusercontent.com/100022747/214065038-16fccd8b-4061-4d07-84b1-42ba80858e9d.PNG)

As stated before, this is merely for simple linear combinations of random variables, not non-linear configurations. So for example, if you wanted to model the distribution of say the product of a Poisson and a Normal distribution, this would not be an available solution for you. I would encourage in that case, to simply model it and appreciate the process of it yourself in your appropriate language!

Likewise, if you happen to be looking for more than 7 inflow and outflow RVs each, you are out of luck unfortunately.

### Inputs

The available choice of distributions are in the drop down menus. Parameters are to be typed in and separated by commas. So for instance, the Cauchy distribution, iirc, requires 2 parameters (eg 1.0,2.0 as a simple example), while something like the Triangular distribution would require 3 parameters (eg 1.0,3.0,2.0).

There is an addition functionality that was deemed essential and was hence added to the parameter bar. For all distributions, regardless of number of parameters required for the actual probability distribution, the user is able to indicate a 4th constant float parameter to multiply with the random variable. As an example, if you wanted to model the potential for the occassional large expense related to for instance, medical bills, and had an estimate of each medical incident in your portfolio costing you about 1000 dollars, at a rate of 0.0166, you would choose ``Poisson`` and type out something along the lines - 0.0166,1.0,1.0,1000.0. The 1.0s inside the parameter are of no significance and could have been any other float values, since Poisson only requires one value, the first value in the Parameter array that we have - the rate of incidence.

### Periods

There is a difference between multiplying a RV with a constant say 3, versus adding it 3 times.  So for example, if you wanted to simulate something akin to 3 years or 3 consecutive months. You could, equivalently, simulate 3 Gaussian RVs in Inflows for instance. Alternatively, you could set ``Periods`` to 3.

### Breakeven

Breakeven is an option for the user to allocate a limit value for the programme to compare against. By default, this is set to 0, and any trials that return values BELOW 0, are analyzed and the contributing variables responsible for that trial, are stored and plotted as boxplots/violin plots for further analysis. You can set this to any other static value. Additionally, if you wish the code to record trials that return values ABOVE the breakeven value, type a ``|`` in the breakeven box, AFTER your static value.


## Output

Various graphs are outputted and saved after a successful simulation. Chief amongst them being, density plots of the individual trials at the END of all periods (if periods are allocated to be above 1), histograms of the same thing, values which are shifted, and logged, and boxplots of contributory RV components to fringe values for the first, middle and end periods are saved as figures.

NOTE: these images are all deleted and replaced upon re-simulation. Be sure to save them somewhere else if valuable to you!

The histogram of the resultant vector of trials and the boxplots for each of the 3 periods are shown as a single plot image at the end.
