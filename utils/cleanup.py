import pandas as pd
import json

carrier_web_replacements = [
   ('various ', ''),    
   ('Various ', ''),  
   ('where A is', ''), 
   ('OPERATORS', ''), 
   ('Full MVNO', ''), 
   ('/', ''), 
   ('Mainly', ''), 
   ('operator reserved', ''), 
   ('All carriers', ''), 
   ('Any carrier', ''), 
   ('O2', 'O2'),
   ('Other mobile networks', ''),
   ('BeeLine', 'Beeline'),   
   ('Sunrise', 'Sunrise'),
   ('Vodafone', 'Vodafone'), 
   ('Orange', 'Orange'), 
   ('WIND', 'WIND'),
   ('Wataniya', 'Wataniya'),
   ('Zain', 'Zain'),
   ('T-Mobile', 'T-Mobile'),
   ('Tele2', 'Tele2'),
   ('TIM', 'TIM'),
   ('mobilkom', 'mobilkom'),
   ('one.vip', 'one.vip'),
   ('eir/Meteor', 'eir/Meteor'),   
   ('YESSSS!', 'YESSSS!'),    
   ('Zapp Mobile', 'Zapp Mobile'),   
   ('Tunisia Tuntel Mobile', 'Tunisie Telecom'),   
   ('Tunisia Mobile Orascom', 'Ooredoo'),   
   ('Tesco', 'Tesco'),
   ('Three', 'Three'),      
   ('Telenor', 'Telenor'),
   ('TCell', 'TCell'),  
   ('Telecel', 'Telecel'),   
   ('Skytel', 'Skytel'),  
   ('Skylink', 'Skylink'),  
   ('Smart Comm', 'Smart Communications'),   
   ('UCell', 'UCell'),
   ('Swisscom', 'Swisscom'),    
   ('one-.vip', 'one.vip'),    
   ('AT&T', 'AT&T'),   
   ('airtel', 'Airtel'),   
   ('Comcel', 'Comcel'),   
   ('LaiLai', 'LaiLai'),   
   ('Lycamobile', 'Lycamobile'),
   ('MTN', 'MTN'),      
   ('Mobilis', 'Mobilis'),  
   ('Nedjma', 'Ooredoo'),  
   ('Korek', 'Korek'),   
   ('Mobinil', 'Orange'),  
   ('Ooredoo', 'Ooredoo'),    
   ('Orascom', 'Ooredoo'),    
   ('Indosat', 'Indosat'), 
   ('Beeline', 'Beeline'),     
   ('H3G (Tre)', 'H3G (Tre)'),     
   ('Globe T', 'Globe Telecom'),   
   ('YESSS!', 'YESSS!'),     
   ('Telkomsel', 'Telkomsel'),     
            
]


def cleanup_prefixes_web():
    with open('INPUT/prefixes.json', 'r') as f:
        data = json.load(f)

    df = pd.DataFrame(data)


    df = df.rename(columns={"carrierName": "carrier_name", "countryName": "country", "callingCode": "CC", "mobilePrefix":"NDC", "fullCode":"prefix"})
    df['carrier_name'] = df['carrier_name'].str.replace(r'\(.*?\)', '', regex=True)

    df['CC'] = df['CC'].str.replace('x', '', regex=False)
    df['NDC'] = df['NDC'].str.replace('x', '', regex=False)
    df['prefix'] = df['prefix'].str.replace('x', '', regex=False)
    df['CC'] = df['CC'].str.replace('+', '', regex=False)
    df['NDC'] = df['NDC'].str.replace('+', '', regex=False)
    df['prefix'] = df['prefix'].str.replace('+', '', regex=False)
    df = df[['country','carrier_name','CC','NDC', 'prefix']]
    df = df.sort_values(by=['country','carrier_name'])
    df.loc[df['country'].str.contains('Sahrawi Arab Democratic Republic', na=False, regex=False), 'country'] = 'Morocco'
    df.loc[df['country'].str.contains('Western Sahara', na=False, regex=False), 'country'] = 'Morocco'    

    df['country'] = df['country'].str.strip()
    df['carrier_name'] = df['carrier_name'].str.strip()
    

    for old_val, new_val in carrier_web_replacements:
        df.loc[df['carrier_name'].str.contains(old_val, na=False, regex=False), 'carrier_name'] = new_val

    df = df[['country','carrier_name','CC','NDC', 'prefix']]
    return df



df_web_prefixes=cleanup_prefixes_web()


def cleanup_prefixes_legacy():
    df = pd.read_excel("ContryCode.xls")
    df = df.drop(columns=['ID'])
    df = df.rename(columns={"PLMNNAME": "carrier_name", "COUNTRY": "country", "COUNTRY_CODE": "CC", "NATIONAL_DESTINATION_CODE":"NDC"})
    df = df[~df['NDC'].str.contains('P', na=False)]
    df['prefix']=''
    df = df[['country','carrier_name','CC','NDC', 'prefix']]

df=cleanup_prefixes_web()
print(df.head(20))


l=sorted(df['carrier_name'].dropna().unique().tolist())
print (l)

print (len(df))