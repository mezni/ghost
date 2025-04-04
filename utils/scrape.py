import os
import pandas as pd
import numpy as np
import requests
from bs4 import BeautifulSoup


def scrape_africa():
    url = "https://en.wikipedia.org/wiki/List_of_mobile_network_operators_in_the_Middle_East_and_Africa"

    response = requests.get(url)
    soup = BeautifulSoup(response.text, "html.parser")

    tables = pd.read_html(url)

    headers = []
    for heading in soup.find_all(['h2', 'h3']):
        headers.append(heading.text.strip())
    headers.remove('Contents') 
    headers.remove('See also') 
    headers.remove('References') 

    table_data = []
    for i, table in enumerate(tables):
        title = headers[i] if i < len(headers) else f"Table {i}"  
        try:
            table = table.drop(columns=["Mobile Prefix"])
        except:
            pass

        input_file = "swap_in.csv"
        output_file = "swap_out.csv"
    
        table.to_csv(input_file, sep="|",index=False)

        with open(input_file, "r", encoding="utf-8") as infile, open(output_file, "w", encoding="utf-8") as outfile:
            for line in infile:
                if "This section needs to be updated." not in line:                    
                    outfile.write(line)

        table = pd.read_csv(output_file, sep="|")
        os.remove(input_file)
        os.remove(output_file)

        if len(table)!=5:
            print (len(table))

scrape_africa()


def scrape_europe():
    url = "https://en.wikipedia.org/wiki/List_of_mobile_network_operators_in_Europe"

    response = requests.get(url)
    soup = BeautifulSoup(response.text, "html.parser")

    tables = pd.read_html(url)

    headers = []
    for heading in soup.find_all(['h2', 'h3']):
        headers.append(heading.text.strip())
    headers.remove('Contents') 
    headers.remove('See also') 
    headers.remove('References') 


    table_data = []
    for i, table in enumerate(tables):
        title = headers[i] if i < len(headers) else f"Table {i}"  # Avoid index errors
        try:
            table = table.drop(columns=["Coverage"])
        except:
            pass

        input_file = "swap_in.csv"
        output_file = "swap_out.csv"
    
        table.to_csv(input_file, sep="|",index=False)

        with open(input_file, "r", encoding="utf-8") as infile, open(output_file, "w", encoding="utf-8") as outfile:
            for line in infile:
                if "Big four mobile network operators" not in line and "Regional mobile network operators" not in line and "Mobile network operators in Cyprus" not in line and "Operators in Abkhazia"  not in line and "Operators in South Ossetia" not in line and "Mobile network operators operating only" not in line:
                    outfile.write(line)

        table = pd.read_csv(output_file, sep="|")
        os.remove(input_file)
        os.remove(output_file)
        if len(table.columns)<6:
            table["xx"]=''

        table.columns = ["Rank", "Operator", "Technology", "Subscribers", "Ownership", "MCCMNC"]        
        table_data.append((title, table))

    i=0
    for title, table in table_data:
        filename = title.replace(" ", "_").replace("/", "-") + ".csv"
        table['Country']=title
        table = table[["Country", "Operator", "Ownership", "MCCMNC", "Technology"]]
        if i==0:
            df_total = table
        else:
            df_total = pd.concat([df_total, table], ignore_index=True)
        i=i+1

    df_total['2G'] = df_total['Technology'].str.contains('GSM') | df_total['Technology'].str.contains('CDMA ')
    df_total['2G'] = df_total['2G'].map({True: 'X', False: ''})
    df_total['GPRS/3G'] = df_total['Technology'].str.contains('UMTS') | df_total['Technology'].str.contains('CDMA2000')
    df_total['GPRS/3G'] = df_total['GPRS/3G'].map({True: 'X', False: ''})
    df_total['4G'] = df_total['Technology'].str.contains('LTE') 
    df_total['4G'] = df_total['4G'].map({True: 'X', False: ''})

    df_total = df_total.drop(columns=["Technology"])


    df_total["Notes"] = df_total["Operator"].str.extract(r"\((.*?)\)")
    df_total["Operator"] = df_total["Operator"].str.replace(r"\s*\(.*?\)", "", regex=True)

    df_total["Includes"] = df_total["Operator"].str.extract(r"(• Includes.*)")
    df_total["Operator"] = df_total["Operator"].str.replace(r"• Includes.*", "", regex=True).str.strip()
    df_total["Notes"] = np.where(
        df_total["Includes"].notna() & (df_total["Includes"] != ""),
        df_total["Notes"].fillna('') + ' | ' + df_total["Includes"],
        df_total["Notes"]
    )

    df_total["Includes"] = df_total["Operator"].str.extract(r"(•Includes.*)")
    df_total["Operator"] = df_total["Operator"].str.replace(r"•Includes.*", "", regex=True).str.strip()
    df_total["Notes"] = np.where(
        df_total["Includes"].notna() & (df_total["Includes"] != ""),
        df_total["Notes"].fillna('') + ' | ' + df_total["Includes"],
        df_total["Notes"]
    )

    df_total["Includes"] = df_total["Operator"].str.extract(r"(• includes.*)")
    df_total["Operator"] = df_total["Operator"].str.replace(r"• includes.*", "", regex=True).str.strip()
    df_total["Notes"] = np.where(
        df_total["Includes"].notna() & (df_total["Includes"] != ""),
        df_total["Notes"].fillna('') + ' | ' + df_total["Includes"],
        df_total["Notes"]
    )

    df_total["Includes"] = df_total["Operator"].str.extract(r"(Coverage: .*)")
    df_total["Operator"] = df_total["Operator"].str.replace(r"Coverage: .*", "", regex=True).str.strip()
    df_total["Notes"] = np.where(
        df_total["Includes"].notna() & (df_total["Includes"] != ""),
        df_total["Notes"].fillna('') + ' | ' + df_total["Includes"],
        df_total["Notes"]
    )
    df_total = df_total.drop(columns=["Includes"])
    df_total["OperatorId"] = df_total["Country"].str.lower() + '_' + df_total["Operator"].str.lower().str.replace(" ", "_")
#    print (df_total.head(20))
#    df_total.to_csv('xxx.csv') 
    df_total.to_csv ('WORK/europe.csv', index=False)

#scrape_europe()