import requests
from bs4 import BeautifulSoup
import pandas as pd

# URL to scrape
url = 'https://en.wikipedia.org/wiki/List_of_mobile_network_operators_in_the_Middle_East_and_Africa'

# Fetch page
response = requests.get(url)
response.raise_for_status()

# Parse HTML
soup = BeautifulSoup(response.text, 'html.parser')

# Find all tables
tables = soup.find_all('table', {'class': 'wikitable'})

# List to collect all scraped data
data = []

# Loop through tables
for table in tables:
    # Get the country name from the previous heading
    country_heading = table.find_previous(['h2', 'h3']).text.strip()
    country = country_heading.replace('[edit]', '').strip()

    # Loop through rows of the table
    for row in table.find_all('tr')[1:]:  # Skip header
        cols = row.find_all('td')
        if len(cols) >= 5:
            operator_id = cols[0].get_text(strip=True)
            operator_name = cols[1].get_text(strip=True)
            operator_tech = cols[2].get_text(strip=True)
            operator_owner = cols[4].get_text(strip=True)

            data.append({
                'country': country,
                'operator_id': operator_id,
                'operator_name': operator_name,
                'operator_tech': operator_tech,
                'operator_owner': operator_owner
            })

# Create DataFrame
df = pd.DataFrame(data)


# Reorder columns if you want
df = df[['country',  'operator_name', 'operator_tech', 'operator_owner']]

print(df)

# Save to CSV (optional)
# df.to_csv('mobile_operators_mea.csv', index=False)
df['operator_name'] = df['operator_name'].str.replace(r'\[.*?\]', '', regex=True).str.strip()
df['operator_name'] = df['operator_name'].str.replace(r'\(.*?\)', '', regex=True).str.strip()
df['operator_name'] = df.apply(lambda row: "" if row['country'] in row['operator_name'] else row['operator_name'], axis=1)
df['operator_tech'] = df['operator_tech'].str.replace(',', ';')
df['operator_owner'] = df['operator_owner'].str.replace(',', ';')


list_of_values = df['operator_name'].tolist()
print (list_of_values)


df.to_csv('mobile_operators_mea.csv', index=False)
