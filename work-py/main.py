import pandas as pd
import phonenumbers 
from phonenumbers import timezone, geocoder, carrier

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



def generate_output(df, output_file): 
    df[['region', 'country', 'operator']] = df['VLR_NUMBER'].apply(lambda x: pd.Series(get_info(x)))
    df.to_csv(output_file, index=False)


file_path = '../../data.csv'

try:
    df_batch = pd.read_csv(file_path)
except pd.errors.EmptyDataError:
    print("Error: CSV file is empty.")


output_file='../../output.csv'
generate_output(df_batch, output_file)