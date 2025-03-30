import pandas as pd

df1 = pd.read_csv('test.txt', sep=' –  ')
df2 = pd.read_csv('cc.csv', delim_whitespace=True)

def test():
    with open('insert.sql', 'w') as f:
        # Write the SQL insert statement for each row in the DataFrame
        for index, row in df.iterrows():
            sql = f"INSERT INTO prefixes (prefix, country_name) VALUES ('{row['prefix']}', '{row['country']}');\n"
            f.write(sql)

df1['prefix'] = df1['prefix'].astype(int)
df2['prefix'] = df2['prefix'].astype(int)

print (df1)
print (df2)
merged_df = pd.merge(df1, df2, on='prefix', how='inner')
print (merged_df)


with open('ins_prefixes.sql', 'w') as f:
        # Write the SQL insert statement for each row in the DataFrame
    for index, row in merged_df.iterrows():
        sql = f"INSERT INTO prefixes (prefix, country_alpha2) VALUES ('{row['prefix']}', '{row['g2']}');\n"
        f.write(sql)