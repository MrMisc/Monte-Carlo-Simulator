import os
import numpy as np
import subprocess
import json
import tkinter as tk
from tkinter.ttk import *
from tkinter import simpledialog
import customtkinter


#Default values
listofdistributions = ["<Distribution>","Gamma", "Uniform", "Normal","Beta","Cauchy","Chi","Chi-Squared","Dirac","Dirichlet","Erlang","Exp","Fisher Snedecor","Geometric","Hyper Geometric","Inverse Gamma","Laplace","Lognormal","Negative Binomial","Poisson","Pareto","StudentsT","Weibull", "Triangular"]
inflows = []
outflows = []
trials = 1000
period = 1
stakeholder = 0.0
morse = "true"



customtkinter.set_appearance_mode("dark")  # Modes: "System" (standard), "Dark", "Light"
customtkinter.set_default_color_theme("dark-blue")  # Themes: "blue" (standard), "green", "dark-blue"


app = customtkinter.CTk()
app.geometry("1650x700")
app.title("Bob")

frame_1 = customtkinter.CTkFrame(master=app)
frame_1.pack(pady=20, padx=60, fill="both", expand=True)
# app.withdraw()
# the input dialog
# USER_INP = simpledialog.askstring(title="Test",
#                                   prompt="What's your Name?:")

# # check it out
# print("Hello", USER_INP)

# def infodump(info, methodology):
#     #methodology is going to be the type of data entry we are considering
#     # we want to generalise data collection and allocation to different variables
#     if methodology == 1:
#         return (method := int(info))
#     elif methodology == 2:
#     else: return None




class App(customtkinter.CTk):
    def __init__(self):
        super().__init__()

        # configure window
        self.title("Generalized Monte Carlo Simulator.py ")
        self.geometry(f"{1650}x{800}")

        # configure grid layout (4x4)
        # self.grid_columnconfigure((0,7),weight=1)
        # self.grid_columnconfigure((1,2,3,4,5,6), weight=7)
        # self.grid_columnconfigure((0,7),weight=1)
        self.grid_columnconfigure((0,1,2,3,4,5,6,7), weight=1)        
        # self.grid_columnconfigure((2, 3), weight=1)
        self.grid_rowconfigure((0, 1,2,3,4,5,6,7), weight=1)
        self.grid_rowconfigure((8), weight=2)

        # create sidebar frame with widgets
        self.sidebar_frame = customtkinter.CTkFrame(self, width=20, corner_radius=25, fg_color = "#141215")
        self.sidebar_frame.grid(row=0, column=0, columnspan = 2,rowspan=10)
        # self.sidebar_frame.grid_rowconfigure(4, weight=1)
        self.logo_label = customtkinter.CTkLabel(self.sidebar_frame, text="Monte Carlo \nSimulator", font=("CF Spaceship", 35))
        self.logo_label.grid(row=0, column=0, padx=10, pady=(10, 0))
        self.sidebar_button_1 = customtkinter.CTkButton(self.sidebar_frame,text = "Simulate" ,command=self.simulate,height=200, width = 400,hover_color="#201925", font=("Microsoft Yi Baiti", 60), corner_radius=20, fg_color = "blueviolet", text_color = "#3A2A46") # Microsoft Yi Baiti,OCR A Extended, Prestige Elite Std
        self.sidebar_button_1.grid(row=4, column=0, rowspan = 4, padx=20, pady=35)

        self.sidebar_trialentry = customtkinter.CTkEntry(self.sidebar_frame,placeholder_text = "  No of Trials" ,font=("OCR A Extended", 25), corner_radius=15, width=300, height= 85, fg_color = "#201925", text_color = "#007FD5") # Microsoft Yi Baiti,OCR A Extended, Prestige Elite Std
        self.sidebar_trialentry.grid(row=1, column=0, rowspan = 1, padx=0, pady=45)    
        self.sidebar_era = customtkinter.CTkEntry(self.sidebar_frame,placeholder_text = "  Periods" ,font=("OCR A Extended", 13), corner_radius=25, width=150, height= 50,fg_color = "#201925", text_color = "#007FD5") # Microsoft Yi Baiti,OCR A Extended, Prestige Elite Std
        self.sidebar_era.grid(row=2, column=0, rowspan = 1, columnspan=2, padx=55, pady=15)  
        self.sidebar_stakeholder = customtkinter.CTkEntry(self.sidebar_frame,placeholder_text = "  Breakeven" ,font=("OCR A Extended", 13), corner_radius=25, width=150, height= 50,fg_color = "#201925", text_color = "#007FD5") # Microsoft Yi Baiti,OCR A Extended, Prestige Elite Std
        self.sidebar_stakeholder.grid(row=3, column=0, columnspan=2, rowspan = 1, padx=55, pady=15)                

        #Inflows
        self.sim1 = customtkinter.CTkLabel(self, text="Inflows", font=("OCR A Extended", 25))
        self.sim1.grid(row=0, column=3, pady=(20,0), columnspan= 2)
        
        self.sim2 = customtkinter.CTkLabel(self, text="Outflows", font=("OCR A Extended", 25))
        self.sim2.grid(row=0, column=5, pady=(20,0), columnspan= 2)

        self.dplus1 = customtkinter.CTkOptionMenu(self, values=listofdistributions,
                                                               font=("Microsoft Yi Baiti", 16), dropdown_font = ("Microsoft Yi Baiti", 14), fg_color = "#8A2BE2", text_color = "tan1")
        self.dplus1.grid(row=1,  columnspan=1, column=3, padx=20, pady=(20, 20))

        self.entry = customtkinter.CTkEntry(self, placeholder_text= "Parameters", font=("Microsoft Yi Baiti", 16),justify = "center", fg_color = "#17131B", text_color = "tan1")  #text inside typing spot
        self.entry.grid(row=1, column=4, columnspan=1,padx=(40, 20), pady=(20, 20)) 
        # self.main_button_1 = customtkinter.CTkButton(master=self, fg_color="transparent", border_width=2, text_color=("gray10", "#DCE4EE"), command=self.button1)
        # self.main_button_1.grid(row=3, column=3, padx=(20, 20), pady=(20, 20))
        self.dplus2 = customtkinter.CTkOptionMenu(self, values=listofdistributions,
                                                               font=("Microsoft Yi Baiti", 16), dropdown_font = ("Microsoft Yi Baiti", 14), fg_color = "#7D2BC8", text_color = "tan1")
        self.dplus2.grid(row=2,  columnspan=1, column=3, padx=20, pady=(20, 20))
        self.entry2 = customtkinter.CTkEntry(self, font=("Microsoft Yi Baiti", 16), justify = "center", fg_color = "#17131B", text_color = "tan1")  #text inside typing spot
        self.entry2.grid(row=2, column=4, columnspan=1, padx=(40, 20), pady=(20, 20)) 

        self.dplus3 = customtkinter.CTkOptionMenu(self, values=listofdistributions,
                                                               font=("Microsoft Yi Baiti", 16), dropdown_font = ("Microsoft Yi Baiti", 14), fg_color = "#6F2BAE", text_color = "tan1")
        self.dplus3.grid(row=3,  columnspan=1, column=3, padx=20, pady=(20, 20))
        self.entry3 = customtkinter.CTkEntry(self, font=("Microsoft Yi Baiti", 16),justify = "center", fg_color = "#17131B", text_color = "tan1")
        self.entry3.grid(row=3, column=4, columnspan=1, padx=(40, 20), pady=(20, 20))         

        self.dplus4 = customtkinter.CTkOptionMenu(self, values=listofdistributions,
                                                               font=("Microsoft Yi Baiti", 16), dropdown_font = ("Microsoft Yi Baiti", 14), fg_color = "#622B94", text_color = "tan1")
        self.dplus4.grid(row=4,  columnspan=1, column=3, padx=20, pady=(20, 20))
        self.entry4 = customtkinter.CTkEntry(self,font=("Microsoft Yi Baiti", 16),justify = "center", fg_color = "#17131B", text_color = "tan1") 
        self.entry4.grid(row=4, column=4, columnspan=1, padx=(40,20), pady=(20, 20))        

        self.dplus5 = customtkinter.CTkOptionMenu(self, values=listofdistributions,
                                                               font=("Microsoft Yi Baiti", 16), dropdown_font = ("Microsoft Yi Baiti", 14), fg_color = "#552A7A", text_color = "tan1")
        self.dplus5.grid(row=5,  columnspan=1, column=3, padx=20, pady=(20, 20))
        self.entry5 = customtkinter.CTkEntry(self,font=("Microsoft Yi Baiti", 16),justify = "center", fg_color = "#17131B", text_color = "tan1") 
        self.entry5.grid(row=5, column=4, columnspan=1, padx=(40,20), pady=(20, 20))                


        self.dplus6 = customtkinter.CTkOptionMenu(self, values=listofdistributions,
                                                               font=("Microsoft Yi Baiti", 16), dropdown_font = ("Microsoft Yi Baiti", 14), fg_color = "#472A60", text_color = "tan1")
        self.dplus6.grid(row=6,  columnspan=1, column=3, padx=20, pady=(20, 20))
        self.entry6 = customtkinter.CTkEntry(self, font=("Microsoft Yi Baiti", 16),justify = "center", fg_color = "#17131B", text_color = "tan1")
        self.entry6.grid(row=6, column=4, columnspan=1, padx=(40, 20), pady=(20, 20))      
        
        self.dplus7 = customtkinter.CTkOptionMenu(self, values=listofdistributions,
                                                               font=("Microsoft Yi Baiti", 16), dropdown_font = ("Microsoft Yi Baiti", 14), fg_color = "#3A2A46", text_color = "tan1")
        self.dplus7.grid(row=7,  columnspan=1, column=3, padx=20, pady=(20, 20))
        self.entry7 = customtkinter.CTkEntry(self, font=("Microsoft Yi Baiti", 16),justify = "center", fg_color = "#17131B", text_color = "tan1")
        self.entry7.grid(row=7, column=4, columnspan=1, padx=(40, 20), pady=(20, 20))                        





    #Outflows
        self.dminus1 = customtkinter.CTkOptionMenu(self, values=listofdistributions,
                                                               font=("Microsoft Yi Baiti", 16), dropdown_font = ("Microsoft Yi Baiti", 14), fg_color = "#8A2BE2", text_color = "tan1")
        self.dminus1.grid(row=1,  columnspan=1, column=5, padx=20, pady=(20, 20))    
        self.entry21 = customtkinter.CTkEntry(self, placeholder_text="Parameters", font=("Microsoft Yi Baiti", 16),justify = "center", fg_color = "#17131B", text_color = "tan1")
        self.entry21.grid(row=1, column=6, columnspan=1, padx=(0,20), pady=(20, 20))  
        # self.main_button_2= customtkinter.CTkButton(master=self, fg_color="transparent", border_width=2, text_color=("gray10", "#DCE4EE"), command=self.button2)
        # self.main_button_2.grid(row=2, column=3, padx=(20, 20), pady=(20, 20))        
        self.dminus2 = customtkinter.CTkOptionMenu(self, values=listofdistributions,
                                                               font=("Microsoft Yi Baiti", 16), dropdown_font = ("Microsoft Yi Baiti", 14), fg_color = "#7D2BC8", text_color = "tan1")
        self.dminus2.grid(row=2,  columnspan=1, column=5, padx=20, pady=(20, 20))    
        self.entry22 = customtkinter.CTkEntry(self, placeholder_text="", font=("Microsoft Yi Baiti", 16),justify = "center", fg_color = "#17131B", text_color = "tan1")  
        self.entry22.grid(row=2, column=6, columnspan=1, padx=(0,20), pady=(20, 20)) 

        self.dminus3 = customtkinter.CTkOptionMenu(self, values=listofdistributions,
                                                               font=("Microsoft Yi Baiti", 16), dropdown_font = ("Microsoft Yi Baiti", 14), fg_color = "#6F2BAE", text_color = "tan1")
        self.dminus3.grid(row=3,  columnspan=1, column=5, padx=20, pady=(20, 20))    
        self.entry23 = customtkinter.CTkEntry(self, placeholder_text="",font=("Microsoft Yi Baiti", 16),justify = "center", fg_color = "#17131B", text_color = "tan1")  
        self.entry23.grid(row=3, column=6, columnspan=1, padx=(0, 20), pady=(20, 20))   

        self.dminus4 = customtkinter.CTkOptionMenu(self, values=listofdistributions,
                                                               font=("Microsoft Yi Baiti", 16), dropdown_font = ("Microsoft Yi Baiti", 14), fg_color = "#622B94", text_color = "tan1")
        self.dminus4.grid(row=4,  columnspan=1, column=5, padx=20, pady=(20, 20))    
        self.entry24 = customtkinter.CTkEntry(self, placeholder_text="",font=("Microsoft Yi Baiti", 16),justify = "center", fg_color = "#17131B", text_color = "tan1")  
        self.entry24.grid(row=4, column=6, columnspan=1, padx=(0, 20), pady=(20, 20))      

        self.dminus5 = customtkinter.CTkOptionMenu(self, values=listofdistributions,
                                                               font=("Microsoft Yi Baiti", 16), dropdown_font = ("Microsoft Yi Baiti", 14), fg_color = "#552A7A", text_color = "tan1")
        self.dminus5.grid(row=5,  columnspan=1, column=5, padx=20, pady=(20, 20))    
        self.entry25 = customtkinter.CTkEntry(self, placeholder_text="",font=("Microsoft Yi Baiti", 16),justify = "center", fg_color = "#17131B", text_color = "tan1")  
        self.entry25.grid(row=5, column=6, columnspan=1, padx=(0, 20), pady=(20, 20))     

        self.dminus6 = customtkinter.CTkOptionMenu(self, values=listofdistributions,
                                                               font=("Microsoft Yi Baiti", 16), dropdown_font = ("Microsoft Yi Baiti", 14), fg_color = "#472A60", text_color = "tan1")
        self.dminus6.grid(row=6,  columnspan=1, column=5, padx=20, pady=(20, 20))    
        self.entry26 = customtkinter.CTkEntry(self, placeholder_text="",font=("Microsoft Yi Baiti", 16),justify = "center", fg_color = "#17131B", text_color = "tan1")  
        self.entry26.grid(row=6, column=6, columnspan=1, padx=(0, 20), pady=(20, 20))                 


        

        self.dminus7 = customtkinter.CTkOptionMenu(self, values=listofdistributions,
                                                               font=("Microsoft Yi Baiti", 16), dropdown_font = ("Microsoft Yi Baiti", 14), fg_color = "#3A2A46", text_color = "tan1")
        self.dminus7.grid(row=7,  columnspan=1, column=5, padx=20, pady=(20, 20))    
        self.entry27 = customtkinter.CTkEntry(self, placeholder_text="",font=("Microsoft Yi Baiti", 16),justify = "center", fg_color = "#17131B", text_color = "tan1")  
        self.entry27.grid(row=7, column=6, columnspan=1, padx=(0, 20), pady=(20, 20))                 


                
    def simulate(self):
        global period
        global stakeholder
        global morse
        dirs = []
        for root, dirs, files in os.walk(".", topdown=False):
            dirs = [name for name in files if name.startswith("Period")]        
        for i in dirs: os.remove(i)
        inflows = []
        outflows = []
        for i in [[self.entry,self.dplus1],[self.entry2,self.dplus2],[self.entry3,self.dplus3],[self.entry4,self.dplus4],[self.entry5,self.dplus5],[self.entry6,self.dplus6],[self.entry7,self.dplus7]]:
            if i[0].get()!="" and i[1].get()!="<Distribution>":
                print(i[0].get())
                inflows.append([i[1].get(),(lambda barelist,varlist: [barelist[i]*varlist[i] if len(varlist)> i else barelist[i] for i in range(len(barelist))])(np.ones(4),[float(x) for x in i[0].get().split(",")])])
        for i in [[self.entry21,self.dminus1],[self.entry22,self.dminus2],[self.entry23,self.dminus3],[self.entry24,self.dminus4],[self.entry25,self.dminus5],[self.entry26,self.dminus6],[self.entry27,self.dminus7]]:
            if i[0].get()!="" and i[1].get()!="<Distribution>":
                # outflows.append([i[1].get(),[float(x) for x in i[0].get().split(",")]])        
                outflows.append([i[1].get(),(lambda barelist,varlist: [barelist[i]*varlist[i] if len(varlist)> i else barelist[i] for i in range(len(barelist))])(np.ones(4),[float(x) for x in i[0].get().split(",")])])
        try:trials = int(self.sidebar_trialentry.get())
        except: pass
        try:period = int(self.sidebar_era.get())
        except: pass
        try:
            global morse
            listo = self.sidebar_stakeholder.get().split("|")
            stakeholder = float(listo[0])
            if len(listo)>1:
                print("Found morse messaging!")
                morse = "false"
        except: morse="true"
        print(f"morse is set to {morse}")
        outputjson = {"inflows":inflows,
        "outflows":outflows,
        "period":period,
        "stakeholder":stakeholder,
        "morse":morse,
        "trials":trials
        }
        with open("./output.json", "w") as outfile:
            json.dump(outputjson, outfile)       
        subprocess.Popen("cargo run | py plot.py", shell=True) 


    def open_input_dialog_event(self):
        dialog = customtkinter.CTkInputDialog(text="Type in a number:", title="CTkInputDialog")
        print("CTkInputDialog:", dialog.get_input())

    def change_scaling_event(self, new_scaling: str):
        new_scaling_float = int(new_scaling.replace("%", "")) / 100
        customtkinter.set_widget_scaling(new_scaling_float)

    def sidebar_button_event(self):
        print("sidebar_button click")


if __name__ == "__main__":
    app = App()
    app.mainloop()