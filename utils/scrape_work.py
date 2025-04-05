import os
import pandas as pd
import numpy as np
import requests
from bs4 import BeautifulSoup

def clean_and_save_table(table, title, output_dir):
    """Common function to clean tables, drop unwanted rows, and save the table to a file."""
    input_file = "swap_in.csv"
    output_file = "swap_out.csv"
    
    # Save table to CSV and filter out unwanted lines
    table.to_csv(input_file, sep="|", index=False)
    
    with open(input_file, "r", encoding="utf-8") as infile, open(output_file, "w", encoding="utf-8") as outfile:
        for line in infile:
            if all(exclusion not in line for exclusion in [
                "Mobile Virtual Network Operators", "Mobile Network Operators",
                "Mobile network operators operating only", "Coverage", "Mainland", 
                "Mobile Prefix", "•Includes", "• includes"
            ]):
                outfile.write(line)
    
    # Load and clean the data
    table = pd.read_csv(output_file, sep="|")
    os.remove(input_file)
    os.remove(output_file)

    # Assign Country and clean specific columns
    table['Country'] = title
    table = table[["Country", "Operator", "Ownership", "MCCMNC", "Technology"]]

    # Handle Subscribers and Technologies
    table["MCCMNC"] = ""
    table["Technology"] = table.get("Technology", '')

    return table

def process_technology_columns(df):
    """Process and classify technology columns (2G, GPRS/3G, 4G)."""
    df['2G'] = df['Technology'].str.contains('GSM') | df['Technology'].str.contains('CDMA ')
    df['2G'] = df['2G'].map({True: 'X', False: ''})
    
    df['GPRS/3G'] = df['Technology'].str.contains('UMTS') | df['Technology'].str.contains('CDMA2000')
    df['GPRS/3G'] = df['GPRS/3G'].map({True: 'X', False: ''})
    
    df['4G'] = df['Technology'].str.contains('LTE') 
    df['4G'] = df['4G'].map({True: 'X', False: ''})
    
    return df.drop(columns=["Technology"])

def process_notes(df):
    """Extract and clean notes from the operator column."""
    df["Notes"] = df["Operator"].str.extract(r"\((.*?)\)")
    df["Operator"] = df["Operator"].str.replace(r"\s*\(.*?\)", "", regex=True)
    
    # Extract and append Includes to Notes
    for pattern in [r"(• Includes.*)", r"(•Includes.*)", r"(• includes.*)", r"(Coverage: .*)"]:
        df["Includes"] = df["Operator"].str.extract(pattern)
        df["Operator"] = df["Operator"].str.replace(pattern, "", regex=True).str.strip()
        df["Notes"] = np.where(df["Includes"].notna() & (df["Includes"] != ""),
                               df["Notes"].fillna('') + ' | ' + df["Includes"], df["Notes"])
    
    df = df.drop(columns=["Includes"])
    return df

def scrape_and_process_data(url):
    output_dir="WORK"
    """Scrape, process, and save data from the provided URL."""
    response = requests.get(url)
    soup = BeautifulSoup(response.text, "html.parser")
    
    tables = pd.read_html(url)
    headers = [heading.text.strip() for heading in soup.find_all(['h2', 'h3'])]
    headers = [h for h in headers if h not in ['Contents', 'See also', 'References']]
    
    table_data = []
    for i, table in enumerate(tables):
        title = headers[i] if i < len(headers) else f"Table {i}"
        
        # Clean and process the table
        cleaned_table = clean_and_save_table(table, title, output_dir)
        cleaned_table = process_technology_columns(cleaned_table)
        cleaned_table = process_notes(cleaned_table)
        
        # Append the cleaned table to the data list
        table_data.append(cleaned_table)
    
    # Concatenate all processed tables
    df_total = pd.concat(table_data, ignore_index=True)
    
    # Generate the OperatorId field
    df_total["OperatorId"] = df_total["Country"].str.lower() + '_' + df_total["Operator"].str.lower().str.replace(" ", "_")
    
    # Save to CSV
    #df_total.to_csv(os.path.join(output_dir, 'combined_data.csv'), index=False)
    return df_total

def main():
    df_europe=scrape_and_process_data("https://en.wikipedia.org/wiki/List_of_mobile_network_operators_in_Europe")
    df_africa=scrape_and_process_data("https://en.wikipedia.org/wiki/List_of_mobile_network_operators_in_the_Middle_East_and_Africa")
    df_asia=scrape_and_process_data("https://en.wikipedia.org/wiki/List_of_mobile_network_operators_in_Asia_and_Oceania")
    df_america=scrape_and_process_data("https://en.wikipedia.org/wiki/List_of_mobile_network_operators_of_the_Americas")
    df_total = pd.concat([df_europe, df_africa, df_asia, df_america], ignore_index=True)
    df_total = df_total.sort_values(by=['Country', 'Operator'])
    df_total.to_csv('WORK/operators.csv', index=False)

main()