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


### Loop the data lines


df = pd.read_csv("./Period10.csv",error_bad_lines=False)
df['Primaries'] = df['Source Type'] + " "+ df['Distribution']

listo = np.unique(list(df['Primaries']))

listofdistributions = ["Gamma", "Uniform", "Normal","Beta","Cauchy","Chi","Chi-Squared","Dirac","Dirichlet","Erlang","Exp","Fisher Snedecor","Geometric","Hyper Geometric","Inverse Gamma","Laplace","Lognormal","Negative Binomial","Poisson","Pareto","StudentsT","Weibull", "Triangular"]

removes = [x for x in listo if x.startswith('outflow ') == False and x.startswith('inflow ')==False or any(stringo in x for stringo in listofdistributions) == False]
df = df[~df['Primaries'].isin(removes)]
df = df.dropna()

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

df = df.groupby('Primaries').filter(lambda x: len(x)>5)
# df_counts = df.groupby(['Primaries']).size().reset_index(name='counts')


# # Draw Stripplot
# fig, ax = plt.subplots(figsize=(16,10), dpi= 80)    
# sns.stripplot(df_counts.Primaries, df_counts.Amount, size=df_counts.counts*2, ax=ax)

# # Decorations
# plt.title('Counts Plot - Size of circle is bigger as more points overlap', fontsize=22)
# plt.show()

# plt.figure(figsize=(13,10), dpi= 80)
# sns.distplot(df.loc[df['Primaries'] == listo[0]], color="dodgerblue", label=listo[0], hist_kws={'alpha':.7}, kde_kws={'linewidth':3})
# sns.distplot(df.loc[df['Primaries'] == listo[1]], color="orange", label=listo[1], hist_kws={'alpha':.7}, kde_kws={'linewidth':3})
# # sns.distplot(df.loc[df['class'] == 'minivan', "cty"], color="g", label="minivan", hist_kws={'alpha':.7}, kde_kws={'linewidth':3})

# plt.title('Density Plot of City Mileage by Vehicle Type', fontsize=22)
# plt.legend()
# plt.show()

def logie(x):
    return np.log(x+1)/np.log(20)

df['Amount'].transform(logie)

plt.figure(figsize=(13,10), dpi= 80)
sns.boxplot(x='Primaries', y='Amount', data=df, hue='Primaries')
sns.stripplot(x='Primaries', y='Amount', data=df, color='black', size=3, jitter=1)

for i in range(len(df['Primaries'].unique())-1):
    plt.vlines(i+.5, 10, 45, linestyles='solid', colors='gray', alpha=0.2)

plt.title('Box Plot of Highway Mileage by Vehicle Class', fontsize=22)
plt.legend(title='Cylinders')
plt.show()