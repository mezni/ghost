import pandas as pd 


df = pd.read_csv('../../../Downloads/country-codes.csv')

#print (df.columns)
#print (df[['Dial','ISO3166-1-Alpha-2','official_name_fr','UNTERM English Short','UNTERM English Formal','official_name_en']])


df = df.rename(columns={
    "ISO3166-1-Alpha-2": "iso",
    "official_name_en": "name_en",
    "official_name_fr": "name_fr",
    "Dial":"prefix"    
})
df['user'] = 'system'
df = df[["iso","name_en","name_fr","prefix","user"]]

print (df.head())

df.to_csv('countries.csv',index=False)