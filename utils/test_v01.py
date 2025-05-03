import pandas as pd

df_networks = pd.read_csv('mcc-mnc.csv', delimiter=';')
df_operators = pd.read_csv('operators.csv')

df_networks['PLMN'] = df_networks['PLMN'].astype(int).astype(str)
#df_networks.loc[df_networks['Brand'].str.contains('zain BH', na=False), 'Brand'] = 'Zain'

print (df_networks.head())
print (df_operators.head())

df_operators['operator_name_upper'] = df_operators['operator_name'].str.upper()
df_operators['country_upper'] = df_operators['country'].str.upper()
df_networks['Brand_upper'] = df_networks['Brand'].str.upper()
df_networks['Country_upper'] = df_networks['Country'].str.upper()



df_operators['operator_name_upper'] = df_operators.apply(
    lambda row: str(row['operator_name_upper']).replace(str(row['country_upper']), '') if pd.notnull(row['country_upper']) and pd.notnull(row['operator_name_upper']) else row['operator_name_upper'],
    axis=1
)


df_networks['Brand_upper'] = df_networks.apply(
    lambda row: str(row['Brand_upper']).replace(str(row['Country_upper']), '') if pd.notnull(row['Country_upper']) and pd.notnull(row['Brand_upper']) else row['Brand_upper'],
    axis=1
)

result = pd.merge(df_operators, df_networks, how='left',  left_on=['operator_name_upper', 'country_upper'], right_on=['Brand_upper', 'Country_upper'])



#merged = pd.merge(df_operators, df_networks, on='country_upper').drop('Country_upper', axis=1)
#result = merged[merged.apply(lambda row: row['operator_name_upper'] in row['Brand_upper'], axis=1)]

result=result[['country','operator_name','operator_owner','notes','2g_flag','3g_flag','lte_flag','MCC','MNC','PLMN','ISO','TADIG']]

print (result.head())
result.to_csv('xxxx.csv', index=False)