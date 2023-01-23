import numpy as np
import glob
import pandas as pd
import csv
import cv2
import json
from matplotlib import rc
from matplotlib.font_manager import FontProperties
from scipy.stats import ks_2samp
import scipy.stats as stats  
import os
import seaborn as sns
import matplotlib
import matplotlib.pyplot as plt
import numpy as np
import subprocess

distr = []
while True:
    try:
        distr.append(float(input()))
    except EOFError as e:
        break

print("Going in to delete")
for root, dirs, files in os.walk(".", topdown=False):
    dirs = [name for name in files if name.startswith("boxplot") or name.startswith("violinplot")]        
print(dirs)
for i in dirs: os.remove(i)


breakeven = open('output.json')
data = json.load(breakeven)
morse = data["morse"]
if morse.__contains__("false"):
    morse = False
else:
    morse = True
stakeholder = data["stakeholder"]


#Special parts
special_data = []
log_data = [np.log(np.abs(min(distr))+1+entry) for entry in distr]
log_special_data = []
if morse:
    special_data = [x for x in distr if x<stakeholder]
    log_special_data = [x for x in log_data if x<np.log(np.abs(min(distr))+1+stakeholder)]
else:
    special_data = [x for x in distr if x>stakeholder]
    log_special_data = [x for x in log_data if x>np.log(np.abs(min(distr))+1+stakeholder)]


prob = float(len(special_data))/float(len(distr))*100.0

#Monte Density plot
#Since this is a density plot, you do not want to show cut up segments of data ontop of each other, their distirbutions would be different

plt.clf()
rc('font',**{'family':'Yu Gothic', 'size': 16})
fig = plt.figure()
plt.style.use('fivethirtyeight') 
plt.figure(figsize=(20,10))
sns.kdeplot(distr , bw_adjust = 0.5 , fill = True, cut = 0, label = "Monte Carlo")
if morse:plt.text(int(min(distr)+max(distr))/2,0.00,f"{prob:.5f}% of seeds, attained units below {stakeholder}", style = "italic", bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
else:plt.text(int(min(distr)+max(distr))/2,0.00,f"{prob:.5f}% of seeds, attained units above {stakeholder}", style = "italic", bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
# sns.kdeplot([x for x in distr if x<0], bw_adjust = 0.5 , fill = True, cut = 0, label = "Monte Carlo")
plt.xlabel("Units", color = 'black')
plt.ylabel("Frequency", color = 'black')
# plt.text(0.1,0.0003,  , bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
# plt.gca().set_xscale("log")
plt.title(f"Distribution of Linear Summation", color = 'black')
# ax.legend(loc='upper right')
plt.savefig(fname='Monte_kde.png')  

#Scaled Monte density plot

plt.clf()
rc('font',**{'family':'Yu Gothic', 'size': 16})
fig = plt.figure()
plt.style.use('fivethirtyeight') 
plt.figure(figsize=(20,10))
sns.kdeplot(log_data , bw_adjust = 0.5 , fill = True, cut = 0, label = "Monte Carlo")
plt.xlabel("Units", color = 'black')
plt.ylabel("Frequency", color = 'black')
# plt.text(0.1,0.0003,  , bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
# plt.gca().set_xscale("log")
plt.title(f"Distribution of Linear Summation", color = 'black')
# ax.legend(loc='upper right')
plt.savefig(fname='Monte_kde_log.png')  


#Monte Histogram

plt.clf()
rc('font',**{'family':'Yu Gothic', 'size': 16})
fig = plt.figure()
plt.style.use('fivethirtyeight') 
plt.figure(figsize=(20,10))
sns.histplot(distr)
sns.histplot(special_data,fill="red", color="darkslateblue")
if morse:plt.text(int(min(distr)+max(distr))/2,0.00,f"{prob:.5f}% of seeds, attained units below {stakeholder}", style = "italic", bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
else:plt.text(int(min(distr)+max(distr))/2,0.00,f"{prob:.5f}% of seeds, attained units above {stakeholder}", style = "italic", bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
plt.xlabel("Units", color = 'black')
plt.ylabel("Frequency", color = 'black')
plt.title(f"Distribution of Linear Summation", color = 'black')
# ax.legend(loc='upper right')
plt.savefig(fname='Monte_hist.png')  


#Monte Histogram_logged
plt.clf()
rc('font',**{'family':'Yu Gothic', 'size': 16})
fig = plt.figure()
plt.style.use('fivethirtyeight') 
plt.figure(figsize=(20,10))
sns.histplot(log_data)
sns.histplot(log_special_data,fill="red", color="darkslateblue")
plt.xlabel("Units", color = 'black')
plt.ylabel("Frequency", color = 'black')
# plt.text(0.1,0.0003,  , bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
# plt.gca().set_xscale("log")
plt.title(f"Distribution of Linear Summation", color = 'black')
# ax.legend(loc='upper right')
plt.savefig(fname='Monte_hist_logged.png')  

#Not kdeplot | Plot 2

# linestyle_tuple = [
#  ((0, (1, 10))),
#  ((0, (1, 1))),
#  ((0, (1, 1))),
#  ((0, (5, 10))),
#  ((0, (5, 5))),
#  ((0, (5, 1))),
#  ((0, (3, 10, 1, 10))),
#  ((0, (3, 5, 1, 5))),
#  ((0, (3, 1, 1, 1))),
#  ((0, (3, 5, 1, 5, 1, 5))),
#  ((0, (3, 10, 1, 10, 1, 10))),
#  ((0, (3, 1, 1, 1, 1, 1)))]

# plt.style.use("fivethirtyeight")
# for param in ['figure.facecolor', 'axes.facecolor', 'savefig.facecolor']:
#     plt.rcParams[param] = '#212946'  # bluish dark grey
# for param in ['text.color', 'axes.labelcolor', 'xtick.color', 'ytick.color']:
#     plt.rcParams[param] = '0.9'  # very light grey
# plt.grid(color='#2A3459')
# # plt.hist(distr)
# plt.title(f"Distribution of Linear Summation", color = 'black')
# plt.subplot(1, 2, 1)
# plt.hist(distr)
# plt.subplot(1, 2, 2)
# plt.hist(distr)
# plt.yscale('log', nonposy='clip')
# plt.savefig(fname='MonteHistogram.png')  









f = open('duration.json')
data = json.load(f)
print("Execution took "+(lambda strset: (str(float(strset)/60.0)+"min") if float(strset)>600.0 else strset + "s")(str(data["Time"])+"."+str(data["Extended Time Details"]*10**-9).split(".")[1]))

# fl = open("./final_resultant_distribution_result")
# writer = csv.writer(fl)
# writer.writerow(distr)
# fl.close()














# print(sum(distr))
# plt.close('all')
# figurine = plt.figure(figsize=(10,9))

# # setting values to rows and column variables
# rows = 2
# columns = 1
  
# # reading images
# Image1 = cv2.imread('tapdataplot.png')
# Image2 = cv2.imread('moneyplot.png')

# figurine.add_subplot(rows, columns, 1)
  
# # showing image
# plt.imshow(Image1)
# plt.axis('off')
# # plt.title("Successes logged by each trial")
  
# # Adds a subplot at the 2nd position
# figurine.add_subplot(rows, columns, 2)
  
# # showing image
# plt.imshow(Image2)
# plt.axis('off')
# # plt.title("Money distribution")

# plt.show()

#------------------------------------------
#Find all the Period files generated

PeriodCounter = len(glob.glob1(".","Period*.csv"))

#Print out the NUMBER of Period files that are named Period[Number].csv
print(PeriodCounter)


#Determine which Period files we will examine. 

if PeriodCounter>2:
    for i in [1,int(PeriodCounter/2),int(PeriodCounter)]:
        strtopath =  f"./Period{i}.csv"
        #Reading Period 1 file
        df = pd.read_csv(strtopath,on_bad_lines='skip')
        # print(list(df))
        # for i in df['Amount']:
        #     try:
        #         float(i)
        #     except:
        #         print("Weird value found!")

        df['Primaries'] = df['Source Type'] + " "+ df['Distribution']

        listo = np.unique(list(df['Primaries']))

        listofdistributions = ["Gamma", "Uniform", "Normal","Beta","Cauchy","Chi","Chi-Squared","Dirac","Dirichlet","Erlang","Exp","Fisher Snedecor","Geometric","Hyper Geometric","Inverse Gamma","Laplace","Lognormal","Negative Binomial","Poisson","Pareto","StudentsT","Weibull", "Triangular"]

        removes = [x for x in listo if x.startswith('outflow ') == False and x.startswith('inflow ')==False or any(stringo in x for stringo in listofdistributions) == False]
        
        print("The list of things to remove are as follows")
        print(removes)

        #Remaining
        # rest = [x for x in listo if x not in removes]

        df = df[~df['Primaries'].isin(removes)]
        df = df.dropna()
        print("ORiginal dataframe looks like this")
        print(df)
        df = df[pd.to_numeric(df['Amount'], errors='coerce').notnull()]
        # df = df.apply(lambda x: pd.to_numeric(x, errors='coerce')).dropna()

        amt = []
        type_of = []
        for thingie in range(len(df["Amount"])):
            try:
                amt.append(np.abs(float(df["Amount"][thingie])))
                # print(amt[-1])
                type_of.append(df['Primaries'][thingie])
            except:
                try:
                    print("Initially, didnt work...")
                    print(type(df["Amount"][thingie]))
                    amt.append(np.abs(float(int(df["Amount"][thingie]))))
                    # print(amt[-1])
                    type_of.append(df['Primaries'][thingie])
                except:pass
        

        data = {'Primaries':type_of,'Amount':amt}
        df = pd.DataFrame(data)

        print("Edited dataframe is ...")
        print(df)

        #Remove entries that do not occur more than 5 times to contribute to the boundary values
        df = df.groupby('Primaries').filter(lambda x: len(x)>5)
        print(f"Final edited dataframe has {len(df)} entries")

        plt.clf()
        plt.style.use("fivethirtyeight")
        if len(df)>5:
            if i==1:sns.boxplot(x = df['Primaries'],y =  df['Amount'])
            else:sns.boxplot(x = df['Primaries'], y = np.log(df['Amount']+1))
        plt.title(f"Primary sources of loss(default)/gain for Period {i}")
        plt.savefig(f"boxplot{i}.png")
        # plt.show()
        if len(df)>5:
            if i==1:sns.violinplot(x = df['Primaries'], y =  df['Amount'])
            else:sns.violinplot(x = df['Primaries'], y = np.log(df['Amount']+1))
        plt.title(f"Primary sources of loss(default)/gain for Period {i}")
        plt.savefig(f"violinplot{i}.png")




    plt.close('all')
    figurine = plt.figure(figsize=(10,9))

    # setting values to rows and column variables
    rows = 2
    columns = 2
    
    # reading images
    Image1 = cv2.imread('Monte_hist.png')
    Image2 = cv2.imread('boxplot1.png')
    Image3 = cv2.imread(f'boxplot{int(PeriodCounter/2)}.png')
    Image4 = cv2.imread(f'boxplot{int(PeriodCounter)}.png')

    figurine.add_subplot(rows, columns, 1)
    
    # showing image
    plt.imshow(Image1)
    plt.axis('off')
    # plt.title("Successes logged by each trial")
    # plt.title(f"Dstribution endpoint analytics, with boxplots for Periods 1, {int(PeriodCounter/2)} and {int(PeriodCounter)}")
    
    # Adds a subplot at the 2nd position
    figurine.add_subplot(rows, columns, 2)
    
    # showing image
    plt.imshow(Image2)
    plt.axis('off')

    figurine.add_subplot(rows, columns, 3)
    
    # showing image
    plt.imshow(Image3)
    plt.axis('off')
    # plt.title("Successes logged by each trial")
    
    # Adds a subplot at the 2nd position
    figurine.add_subplot(rows, columns, 4)
    
    # showing image
    plt.imshow(Image4)
    plt.axis('off')    
    
    plt.show()
else:
    strtopath =  f"./Period{PeriodCounter}.csv"
    #Reading Period 1 file
    df = pd.read_csv(strtopath,on_bad_lines='skip')
    # print(list(df))
    # for i in df['Amount']:
    #     try:
    #         float(i)
    #     except:
    #         print("Weird value found!")

    df['Primaries'] = df['Source Type'] + " "+ df['Distribution']

    listo = np.unique(list(df['Primaries']))

    listofdistributions = ["Gamma", "Uniform", "Normal","Beta","Cauchy","Chi","Chi-Squared","Dirac","Dirichlet","Erlang","Exp","Fisher Snedecor","Geometric","Hyper Geometric","Inverse Gamma","Laplace","Lognormal","Negative Binomial","Poisson","Pareto","StudentsT","Weibull", "Triangular"]

    removes = [x for x in listo if x.startswith('outflow ') == False and x.startswith('inflow ')==False or any(stringo in x for stringo in listofdistributions) == False]
    
    print("The list of things to remove are as follows")
    print(removes)

    #Remaining
    # rest = [x for x in listo if x not in removes]

    df = df[~df['Primaries'].isin(removes)]
    df = df.dropna()
    print("ORiginal dataframe looks like this")
    print(df)
    df = df[pd.to_numeric(df['Amount'], errors='coerce').notnull()]
    # df = df.apply(lambda x: pd.to_numeric(x, errors='coerce')).dropna()

    amt = []
    type_of = []
    for thingie in range(len(df["Amount"])):
        try:
            amt.append(np.abs(float(df["Amount"][thingie])))
            # print(amt[-1])
            type_of.append(df['Primaries'][thingie])
        except:
            try:
                print("Initially, didnt work...")
                print(type(df["Amount"][thingie]))
                amt.append(np.abs(float(int(df["Amount"][thingie]))))
                # print(amt[-1])
                type_of.append(df['Primaries'][thingie])
            except:pass
    

    data = {'Primaries':type_of,'Amount':amt}
    df = pd.DataFrame(data)

    print("Edited dataframe is ...")
    print(df)

    #Remove entries that do not occur more than 5 times to contribute to the boundary values
    df = df.groupby('Primaries').filter(lambda x: len(x)>5)
    print(f"Final edited dataframe has {len(df)} entries")

    plt.clf()
    plt.style.use("fivethirtyeight")
    if len(df)>5:
        if i==1:sns.boxplot(df['Primaries'], df['Amount'])
        else:sns.boxplot(df['Primaries'], np.log(df['Amount']+1))
    plt.title(f"Primary sources of loss(default)/gain for Period {i}")
    plt.savefig(f"boxplot{PeriodCounter}.png")
    # plt.show()
    if len(df)>5:
        if i==1:sns.violinplot(df['Primaries'],  df['Amount'])
        else:sns.violinplot(df['Primaries'], np.log(df['Amount']+1))
    plt.title(f"Primary sources of loss(default)/gain for Period {i}")
    plt.savefig(f"violinplot{PeriodCounter}.png")    
    plt.close('all')
    figurine = plt.figure(figsize=(10,9))

    rows = 1
    columns = 2
    
    # reading images
    Image1 = cv2.imread('Monte_hist.png')
    Image2 = cv2.imread(f'boxplot{int(PeriodCounter)}.png')

    figurine.add_subplot(rows, columns, 1)
    
    # showing image
    plt.imshow(Image1)
    plt.axis('off')
    # plt.title("Successes logged by each trial")
    # plt.title(f"Dstribution endpoint analytics, with boxplots for Periods 1, {int(PeriodCounter/2)} and {int(PeriodCounter)}")
    
    # Adds a subplot at the 2nd position
    figurine.add_subplot(rows, columns, 2)
    
    # showing image
    plt.imshow(Image2)
    plt.axis('off')
    plt.show()
