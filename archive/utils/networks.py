import pandas as pd

df = pd.read_csv('mcc-mnc.csv', delimiter=';', dtype=str)

df = df.rename(columns={'Operator': 'Ownership'})
df = df.rename(columns={'Brand': 'Operator'})


df['tech_2g'] = df['Bands'].str.contains('GSM', case=False, na=False).map({True: 'X', False: ''})
df['tech_3g'] = df['Bands'].str.contains('UMTS', case=False, na=False).map({True: 'X', False: ''})
df['tech_lte'] = df['Bands'].str.contains('LTE', case=False, na=False).map({True: 'X', False: ''})

#df['country_code'] = df['Country'].str.lower().str.replace(' ', '_')

df = df [['Country','Operator', 'Ownership', 'PLMN','MCC', 'MNC', 'TADIG', 'ISO','tech_2g','tech_3g','tech_lte']]
df = df.sort_values(by=['Country', 'Operator'])

#df.to_csv('networks.csv', index=False)

df_networks=df
df_countries = pd.read_csv('countries.csv')


df_joined = df_networks.merge(
    df_countries,
    left_on=['ISO'],
    right_on=['iso'],
    how='left'
)

df_joined = df_joined [['common_name','Operator', 'Ownership', 'PLMN','MCC', 'MNC', 'TADIG', 'ISO','tech_2g','tech_3g','tech_lte']]
df_joined = df_joined[df_joined['common_name'].notnull()]
df_joined = df_joined[df_joined['Operator'].notnull()]
#df_joined = df_joined[(df_joined['tech_2g'] == 'X') | (df_joined['tech_3g'] == 'X') | (df_joined['tech_lte'] == 'X')]
df_joined = df_joined.rename(columns={'common_name': 'Country'})
df_joined = df_joined [['Country','Operator', 'Ownership', 'PLMN','MCC', 'MNC', 'TADIG', 'ISO','tech_2g','tech_3g','tech_lte']]


df_joined.to_csv('networks.csv', index=False)