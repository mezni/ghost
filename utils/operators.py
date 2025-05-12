import pandas as pd

df = pd.read_csv('operators.csv',dtype=str)

df_operators=df
df_countries = pd.read_csv('countries.csv')


df_joined = df_operators.merge(
    df_countries,
    left_on=['Country'],
    right_on=['common_name'],
    how='left'
)

df_joined = df_joined [['common_name','Operator', 'Brand', 'PLMN']]
df_joined = df_joined[df_joined['common_name'].notnull()]
df_joined = df_joined[df_joined['Operator'].notnull()]
df_joined = df_joined.rename(columns={'common_name': 'Country'})
df_joined = df_joined [['Country','Operator', 'Brand', 'PLMN']]


df_joined.to_csv('xxxxx.csv', index=False)