import os
import pandas as pd
import numpy as np
import requests
from bs4 import BeautifulSoup

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



def scrape_africa():
    url = "https://en.wikipedia.org/wiki/List_of_mobile_network_operators_in_the_Middle_East_and_Africa"

    response = requests.get(url)
    soup = BeautifulSoup(response.text, "html.parser")

    tables = pd.read_html(url)

    headers = []
    for heading in soup.find_all(['h2', 'h3']):
        headers.append(heading.text.strip())
    headers = [h for h in headers if h not in ['Contents', 'See also', 'References']]


    table_data_table = []
    table_data_title = []    
    for i, table in enumerate(tables):
        title = headers[i] if i < len(headers) else f"Table {i}"  
        table_data_title.append(title)
        table_data_table.append(table)

    filtered_titles = []
    for title in table_data_title:
        if "Table" in title:
            continue  # Skip if title contains "Table"
        filtered_titles.append(title)


    filtered_tables = []
    for table in table_data_table:
        if len(table.columns) <= 2:
            continue  # Skip if title contains "Table"
        filtered_tables.append(table)

    table_data=[]
    for title, table in zip(filtered_titles, filtered_tables):
        try:
            table = table.drop(columns=["Mobile Prefix"])
        except:
            pass

        try:            
            table.rename(columns={'Subscribers[64] (in millions)': 'Subscribers (in millions)'}, inplace=True)
        except:
            pass
        table = table.drop(columns=["Subscribers (in millions)"])
        table["MCCMNC"]=""
        table_data.append((title, table))
 
    i=0
    for title, table in table_data:

        input_file = "swap_in.csv"
        output_file = "swap_out.csv"
    
        table.to_csv(input_file, sep="|",index=False)

        with open(input_file, "r", encoding="utf-8") as infile, open(output_file, "w", encoding="utf-8") as outfile:
            for line in infile:
                if "Mobile Virtual Network Operators" not in line and "Mobile Network Operators" not in line :
                    outfile.write(line)

        table = pd.read_csv(output_file, sep="|")
        os.remove(input_file)
        os.remove(output_file)



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
    df_total.to_csv ('WORK/africa.csv', index=False)


def scrape_asia():
    url = "https://en.wikipedia.org/wiki/List_of_mobile_network_operators_in_Asia_and_Oceania"

    response = requests.get(url)
    soup = BeautifulSoup(response.text, "html.parser")

    tables = pd.read_html(url)

    headers = []
    for heading in soup.find_all(['h2', 'h3']):
        headers.append(heading.text.strip())
    headers = [h for h in headers if h not in ['Contents', 'See also', 'References']]


    table_data_table = []
    table_data_title = []    
    for i, table in enumerate(tables):
        title = headers[i] if i < len(headers) else f"Table {i}"  
        table_data_title.append(title)
        table_data_table.append(table)

    filtered_titles = []
    for title in table_data_title:
        if "Table" in title:
            continue  # Skip if title contains "Table"
        filtered_titles.append(title)


    filtered_tables = []
    for table in table_data_table:
        if len(table.columns) <= 2:
            continue  # Skip if title contains "Table"
        filtered_tables.append(table)

    table_data=[]
    for title, table in zip(filtered_titles, filtered_tables):
        try:
            table = table.drop(columns=["Mobile Prefix"])
        except:
            pass

        input_file = "swap_in.csv"
        output_file = "swap_out.csv"
    
        table.to_csv(input_file, sep="|",index=False)

        with open(input_file, "r", encoding="utf-8") as infile, open(output_file, "w", encoding="utf-8") as outfile:
            for line in infile:
                if "Mainland Pakistan" not in line and "Mobile Virtual Network Operators" not in line and "AJK and Gilgit-Baltistan" not in line and "Operators in Abkhazia"  not in line and "Operators in South Ossetia" not in line and "Mobile network operators operating only" not in line:
                    outfile.write(line)

        table = pd.read_csv(output_file, sep="|")
        os.remove(input_file)
        os.remove(output_file)

        print (table)
        try:            
            table.rename(columns={'Subscribers[38] (in millions)': 'Subscribers (in millions)'}, inplace=True)
            table.rename(columns={'Subscribers': 'Subscribers (in millions)'}, inplace=True)   
            table.rename(columns={'Subscribers (in millions, not precise)': 'Subscribers (in millions)'}, inplace=True)
            table.rename(columns={'Subscribers (in millions)': 'Subscribers (in millions)'}, inplace=True) 
            table.rename(columns={'Subscribers (in millions)': 'Subscribers (in millions)'}, inplace=True) 
        except:
            pass
        print (title)
        print (table.columns)
        table = table.drop(columns=["Subscribers (in millions)"])
        table["MCCMNC"]=""
        if "Technology" not in table.columns:
            table["Technology"]=''
        table_data.append((title, table))
 
    i=0
    for title, table in table_data:

        input_file = "swap_in.csv"
        output_file = "swap_out.csv"
    
        table.to_csv(input_file, sep="|",index=False)

        with open(input_file, "r", encoding="utf-8") as infile, open(output_file, "w", encoding="utf-8") as outfile:
            for line in infile:
                if "Mobile Virtual Network Operators" not in line and "Mobile Network Operators" not in line :
                    outfile.write(line)

        table = pd.read_csv(output_file, sep="|")
        os.remove(input_file)
        os.remove(output_file)

        filename = title.replace(" ", "_").replace("/", "-") + ".csv"
        table['Country']=title
        table = table[["Country", "Operator", "Ownership", "MCCMNC", "Technology"]]
        if i==0:
            df_total = table
        else:
            df_total = pd.concat([df_total, table], ignore_index=True)
        i=i+1
        

    df_total["Country"] = df_total["Country"].str.replace(r"\(mainland\)", "", regex=True).str.strip()

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
    df_total.to_csv ('WORK/asia.csv', index=False)


def scrape_america():
    url = "https://en.wikipedia.org/wiki/List_of_mobile_network_operators_of_the_Americas"

    response = requests.get(url)
    soup = BeautifulSoup(response.text, "html.parser")

    tables = pd.read_html(url)

    headers = []
    for heading in soup.find_all(['h2', 'h3']):
        headers.append(heading.text.strip())
    headers = [h for h in headers if h not in ['Contents', 'See also', 'References']]


    table_data_table = []
    table_data_title = []    
    for i, table in enumerate(tables):
        title = headers[i] if i < len(headers) else f"Table {i}"  
        table_data_title.append(title)
        table_data_table.append(table)

    filtered_titles = []
    for title in table_data_title:
        if "Table" in title:
            continue  # Skip if title contains "Table"
        filtered_titles.append(title)


    filtered_tables = []
    for table in table_data_table:
        if len(table.columns) <= 2:
            continue  # Skip if title contains "Table"
        filtered_tables.append(table)

    table_data=[]
    for title, table in zip(filtered_titles, filtered_tables):
        try:
            table = table.drop(columns=["Mobile Prefix"])
        except:
            pass

        input_file = "swap_in.csv"
        output_file = "swap_out.csv"
    
        table.to_csv(input_file, sep="|",index=False)

        with open(input_file, "r", encoding="utf-8") as infile, open(output_file, "w", encoding="utf-8") as outfile:
            for line in infile:
                if "Mobile Virtual Network Operators" not in line :
                    outfile.write(line)

        table = pd.read_csv(output_file, sep="|")
        os.remove(input_file)
        os.remove(output_file)

        print (table)
        try:            
            table.rename(columns={'Subscribers (in millions)[50]': 'Subscribers (in millions)'}, inplace=True)
            table.rename(columns={'Subscribers (in millions) markets share (%)': 'Subscribers (in millions)'}, inplace=True)   
        except:
            pass
        print (title)
        print (table.columns)
        table = table.drop(columns=["Subscribers (in millions)"])
        table["MCCMNC"]=""
        if "Technology" not in table.columns:
            table["Technology"]=''
        table_data.append((title, table))
 
    i=0
    for title, table in table_data:

        input_file = "swap_in.csv"
        output_file = "swap_out.csv"
    
        table.to_csv(input_file, sep="|",index=False)

        with open(input_file, "r", encoding="utf-8") as infile, open(output_file, "w", encoding="utf-8") as outfile:
            for line in infile:
                if "Mobile Virtual Network Operators" not in line and "Mobile Network Operators" not in line :
                    outfile.write(line)

        table = pd.read_csv(output_file, sep="|")
        os.remove(input_file)
        os.remove(output_file)

        filename = title.replace(" ", "_").replace("/", "-") + ".csv"
        table['Country']=title
        table = table[["Country", "Operator", "Ownership", "MCCMNC", "Technology"]]
        if i==0:
            df_total = table
        else:
            df_total = pd.concat([df_total, table], ignore_index=True)
        i=i+1
        

    df_total["Country"] = df_total["Country"].str.replace(r"\(mainland\)", "", regex=True).str.strip()

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
    df_total.to_csv ('WORK/america.csv', index=False)


scrape_europe()
scrape_africa()
scrape_asia()
scrape_america()



df_europe=pd.read_csv("WORK/europe.csv")
df_africa=pd.read_csv("WORK/africa.csv")
df_asia=pd.read_csv("WORK/asia.csv")
df_america=pd.read_csv("WORK/america.csv")
df_total = pd.concat([df_europe, df_africa, df_asia, df_america], ignore_index=True)
df_total = df_total.sort_values(by=['Country', 'Operator'])
df_total.to_csv('WORK/operators.csv', index=False)