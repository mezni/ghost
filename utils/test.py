import pandas as pd

df = pd.read_csv('INPUT/prefixes.csv')
#df = df.rename(columns={'alpha2': 'country_alpha2'})
print (df)
df = df.sort_values(by='prefix')


with open('ins_prefixes.sql', 'w') as f:
    for index, row in df.iterrows():
        sql = f"INSERT INTO countries (prefix, country_alpha2) VALUES ('{row['prefix']}', '{row['country_alpha2']}');\n"
        f.write(sql)