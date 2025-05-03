import requests
from bs4 import BeautifulSoup
import pandas as pd

def scrape_mobile_operators(url):


    try:
        response = requests.get(url, timeout=10)
        response.raise_for_status()
        soup = BeautifulSoup(response.text, 'html.parser')
    except requests.RequestException as e:
        print(f"Error fetching page: {e}")
        return pd.DataFrame()

    tables = soup.find_all('table', {'class': 'wikitable'})
    all_dfs = []

    for i, table in enumerate(tables):
        # Get country from caption or previous heading
        caption = table.find('caption')
        if caption:
            title = caption.get_text(strip=True)
        else:
            prev_heading = table.find_previous(['h2', 'h3'])
            title = prev_heading.get_text(strip=True).replace("[edit]", "") if prev_heading else f"Unknown {i+1}"

        country_name = title.split('[')[0].strip()
        # print(f"Processing country: {country_name}")

        # Extract headers
        headers = []
        for th in table.find('tr').find_all('th'):
            header_text = th.get_text(strip=True)
            if "Subscribers" in header_text or "Subscrbiers" in header_text:
                header_text = "Subscribers"
            elif "Technology" in header_text:
                header_text = "Technology"
            elif "MCC" in header_text or "MNC" in header_text:
                header_text = "PLMN"
            headers.append(header_text)

        # Extract data rows
        data = []
        for row in table.find_all('tr')[1:]:
            row_data = [td.get_text(' ', strip=True) for td in row.find_all(['td', 'th'])]
            data.append(row_data)

        # Create DataFrame if column count matches
        if data and len(data[0]) == len(headers):
            df = pd.DataFrame(data, columns=headers)
            df['SourceTable'] = title
            df['Country'] = country_name

            # Add standard columns if missing
            for col in ['Operator', 'Brand', 'Technology', 'Subscribers', 'PLMN']:
                if col not in df.columns:
                    df[col] = ""

            df['Technology'] = df['Technology'].str.replace(',', ';') 
            df['Ownership'] = df['Ownership'].str.replace(',', ';')        
            df['Operator'] = df['Operator'].str.replace(r'\[.*?\]', '', regex=True).str.strip()
            df['Operator'] = df['Operator'].str.replace(r'\(.*?\)', '', regex=True).str.strip()
            df['Operator'] = df['Operator'].str.replace(r'•.*', '', regex=True).str.strip()
            df['Operator'] = df['Operator'].str.replace(r'\*.*', '', regex=True).str.strip()
            df['Operator'] = df['Operator'].str.replace(r'Includes.*', '', regex=True).str.strip()      
            df['Operator'] = df['Operator'].str.strip()

            df['tech_2g'] = df['Technology'].str.contains('GSM', case=False, na=False).map({True: 'X', False: ''})
            df['tech_3g'] = df['Technology'].str.contains('UMTS', case=False, na=False).map({True: 'X', False: ''})
            df['tech_lte'] = df['Technology'].str.contains('LTE', case=False, na=False).map({True: 'X', False: ''})

            all_dfs.append(df)

    if all_dfs:
        final_df = pd.concat(all_dfs, ignore_index=True)

        # Filter out MVNOs
        mvno_keywords = ['MVNO', 'Mobile Virtual', 'Virtual Network', 'Virtual Operator']
        mask = ~final_df.apply(lambda row: row.astype(str).str.contains(
            '|'.join(mvno_keywords), case=False).any(), axis=1)
        final_df = final_df[mask]

        # Reorder columns
        desired_columns_order = ['Country', 'Operator', 'Brand', 'PLMN', 'tech_2g', 'tech_3g','tech_lte']


        return final_df[desired_columns_order]

    return pd.DataFrame()


urls = [ 
'https://en.wikipedia.org/wiki/List_of_mobile_network_operators_in_the_Middle_East_and_Africa',
'https://en.wikipedia.org/wiki/List_of_mobile_network_operators_in_Europe',
'https://en.wikipedia.org/wiki/List_of_mobile_network_operators_in_Asia_and_Oceania',
'https://en.wikipedia.org/wiki/List_of_mobile_network_operators_of_the_Americas'
]

i=0
for url in urls:
    df = scrape_mobile_operators(url)
    if i==0:
        df_combined = df
    else:
        df_combined = pd.concat([df_combined, df], ignore_index=True)
    i=i+1

df_operators=df_combined
#df_operators['country_upper'] = df_operators['Country'].str.upper()
#df_operators['operator_upper'] = df_operators['Operator'].str.upper()
df_operators=df_operators[['Country','Operator','PLMN','tech_2g','tech_3g','tech_lte']]
df_operators = df_operators.sort_values(by=['Country', 'Operator'])
df_operators.to_csv('operators.csv', index=False)


df_networks = pd.read_csv('mcc-mnc.csv', delimiter=';')
#df_networks['country_upper'] = df_networks['Country'].str.upper()
#df_networks['operator_upper'] = df_networks['Brand'].str.upper()
df_networks = df_networks[['Country','Operator','PLMN','MCC','MNC','TADIG','ISO']]
df_networks = df_networks.sort_values(by=['Country', 'Operator'])
df_networks.to_csv('networks.csv', index=False)


