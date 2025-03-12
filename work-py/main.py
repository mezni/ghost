import pandas as pd
import phonenumbers 
from phonenumbers import timezone, geocoder, carrier
file_path = '../../data.csv'

try:
    df_batch = pd.read_csv(file_path)
except pd.errors.EmptyDataError:
    print("Error: CSV file is empty.")


def get_info(msisdn):
    region=None
    country=None
    operator=None
    msisdn=str(msisdn)
    if msisdn.startswith('+'):
        phone_number=msisdn
    else:
        phone_number='+'+msisdn
    
    
    try:
        parsed_number = phonenumbers.parse(phone_number, None)
        region = geocoder.description_for_number(parsed_number, "fr")
        operator = carrier.name_for_number(parsed_number, "fr")
        country = phonenumbers.region_code_for_number(parsed_number)       
    except:
        pass
    
    return region,country,operator

#region,country,operator = get_info('+21697501501')
#print (region,country,operator)


#df = df_batch[df_batch['VLR_NUMBER'].map(lambda x: str(x).startswith('33'))]
#print(df.head(20))
#print(df.size)


df_batch[['region', 'country', 'operator']] = df_batch['VLR_NUMBER'].apply(lambda x: pd.Series(get_info(x)))
#print(df_batch.head(20))
#print(df_batch.size)
#df_batch.to_csv('output.csv', index=False)

#df_batch=df_batch.groupby(['country','operator'])['MSISDN'].count()
#print(df_batch.head(20))


df=df_batch.copy()
grouped_df = df.groupby(['operator']).size().reset_index(name='counts')

# Sort the results by Category and then by Value
sorted_df = grouped_df.sort_values(by=['counts'], ascending=False)
print(sorted_df.head(10))


df=df_batch.copy()
grouped_df = df.groupby(['country']).size().reset_index(name='counts')

# Sort the results by Category and then by Value
sorted_df = grouped_df.sort_values(by=['counts'], ascending=False)
print(sorted_df.head(10))


df=df_batch.copy()
grouped_df = df.groupby(['region']).size().reset_index(name='counts')

# Sort the results by Category and then by Value
sorted_df = grouped_df.sort_values(by=['counts'], ascending=False)
print(sorted_df.head(10))