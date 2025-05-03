import requests
from bs4 import BeautifulSoup
import pandas as pd

def scrape_mobile_operators():
    url = "https://en.wikipedia.org/wiki/List_of_mobile_network_operators_of_the_Americas"
    
    # Fetch the page
    response = requests.get(url)
    soup = BeautifulSoup(response.text, 'html.parser')

    # Find all tables with mobile operators
    tables = soup.find_all('table', {'class': 'wikitable'})
    all_dfs = []

    for i, table in enumerate(tables):
        # Extract table caption (title)
        caption = table.find('caption')
        title = caption.get_text(strip=True) if caption else f"Table {i+1}"
        
        # Extract and correct column headers
        headers = []
        for th in table.find('tr').find_all('th'):
            header_text = th.get_text(strip=True)
            # Standardize column names
            if "Subscribers" in header_text or "Subscribers" in header_text:
                header_text = "Subscribers"
            if "MCC" in header_text or "MCC" in header_text:
                header_text = "Plmn"
            headers.append(header_text)
        
        # Extract table data
        data = []
        for row in table.find_all('tr')[1:]:
            row_data = [td.get_text(strip=True) for td in row.find_all(['td', 'th'])]
            data.append(row_data)
        
        # Create DataFrame if structure is valid
        if len(data) > 0 and len(data[0]) == len(headers):
            df = pd.DataFrame(data, columns=headers)
            
            # Add source column
            df['Source'] = title
            
            # Check if MCC/MNC columns exist
            mcc_missing = not any(col in df.columns for col in ['Plmn'])
            
            if mcc_missing:
                df['Plmn'] = ""
            
            all_dfs.append(df)

    # Combine all DataFrames
    if all_dfs:
        final_df = pd.concat(all_dfs, ignore_index=True)
        
        # Reorder columns to put MCC/MNC and Source at the end
        cols = [col for col in final_df.columns if col not in ['Plmn', 'Source']]
        cols.extend(['Plmn', 'Source'])
        final_df = final_df[cols]
        
        return final_df
    else:
        return pd.DataFrame()

# Run the scraper
mobile_operators_df = scrape_mobile_operators()

# Display results
print(f"Total operators found: {len(mobile_operators_df)}")
print("\nFirst 5 operators:")
print(mobile_operators_df.head())
print("\nLast 5 operators:")
print(mobile_operators_df.tail())
print("\nColumn names:", mobile_operators_df.columns.tolist())

# Save to CSV (optional)
mobile_operators_df.to_csv('mobile_operators_americas.csv', index=False)
print("\nData saved to 'mobile_operators_americas.csv'")