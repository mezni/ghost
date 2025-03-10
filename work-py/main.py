import pandas as pd
file_path = '../../data.csv'

try:
    df_batch = pd.read_csv(file_path)
except pd.errors.EmptyDataError:
    print("Error: CSV file is empty.")

print(df_batch.head())
print(df_batch.size)

df=df_batch.groupby(['VLR_NUMBER'])['MSISDN'].count()
print(df.head())


df = df_batch[df_batch['VLR_NUMBER'].map(lambda x: str(x).startswith('33'))]
print(df.head(20))
print(df.size)