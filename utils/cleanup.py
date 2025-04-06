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


country_replacements = [
    
('BUTSWANA','Botswana'),
('BOSNIA','Bosnia & Herzegovina'),
('IRELAND','Ireland'),
('BOLIVIA','Bolivia, Plurinational State Of'),
('TIMOR','East Timor'),
('SWITZERL','Switzerland'), 
('SALVADOR','El Salvador'), 
('SYRIA','Syrian Arab Republic'),  
('TONGA','Tonga'),        
('TANZANIA','Tanzania, United Republic Of'),
('TCHAD','Chad'),
('TRINIDAD','Trinidad And Tobago'),
('VENEZUELA','Venezuela, Bolivarian Republic Of'),
('VIETNAM','Viet Nam'),
('UK','United Kingdom'),
('ROYAUME-UNI','United Kingdom'),
('USA','United States'),
('TUNIS','Tunisia'),
('TUNISIE','Tunisia'),
('TAJAKISTAN','Tajikistan'),
('TAIWAN','Taiwan'),
('SWITZELAND','Switzerland'),
('SURINAM','Suriname'),
('SURINAM','Suriname'),
('SUREGUER','Guernsey'),
('SOUTHAFRICA','South Africa'),
('SLOVAK','Slovakia'),
('SIERRALEONE','Sierra Leone'),
('REPUBLIQUE CHEQUE','Czech Republic'),
('ROAMANIA','Romania'),
('RWANDA','Rwanda'),
('REUNION','Reunion'),
('NORWAY','Norway'),
('NLECALE','New Caledonia'),
('NEWGUINEA','Papua New Guinea'),
('NEW GUINEA','Papua New Guinea'),
('MYANMA','Myanmar'),
('MOZAMBI','Mozambique'),
('INDIA','India'),
('MOLDOVA','Moldova'),
('MOLDAVIA','Moldova'),
('MAURITAN','Mauritania'),
('MALTE','Malta'),
('MACEDONIA','Macedonia'),
('LIBYE','Libya'),
('INDONESI','Indonesia'),
('IVORYCOAST','Côte d''Ivoire'),
('HONGKONG','Hong Kong'),
('GUYANE','Guyana'),
('GUINEEB','Guinea-Bissau'),
('GUINEAB','Guinea-Bissau'),
('GUINEE','Guinea'),
('ETHIOP','Ethiopia'),
('FIJI','Fiji'),
('GABON','Gabon'),
('GHABON','Gabon'),
('DEUTSCHLAND','Germany'),
('EGYPT','Egypt'),
('EQUATORIALGUINEA','Equatorial Guinea'),
('COSTARICA','Costa Rica'),
('IRAN','Iran, Islamic Republic Of'),
('UZBEKISTAN','Uzbekistan'),
('THAILAND','Thailand'),
('CHINE','China'),
('COMBODGE','Cambodia'),
('CAMBODGE','Cambodia'),
('SOUTH SUDAN','South Sudan'),
('KITTS','Saint Kitts And Nevis'),
('SAOTOME','São Tomé and Príncipe'),
('RUSSIA','Russian Federation'),
('PALESTINE','Palestine, State of'),
('CENTRAL AFRIC','Central African Republic'),
('CAPVERT','Cabo Verde'),
('CAP VERD','Cabo Verde'),
('CAMEROUN','Cameroon'),
('AZERBAIJAN','Azerbaijan'),
('BORKINAFASO','Burkina Faso'),
('ANTIGUA','Antigua And Barbuda'),
('VIRGIN ISL','Virgin Islands (British)'),
('VIRGINISL','Virgin Islands (British)'),
('CANADA','Canada'),
('CONGO RDC','Democratic Republic Of Congo'),
('CAICOS','Turks And Caicos Islands'),
('BRUNEI','Brunei Darussalam'),
('COOK','Cook Islands'),
('CONGO [DRC', 'Democratic Republic Of Congo'),
('CONGO [REPUBLIC', 'Republic Of Congo'),
('ANGILLA', 'Anguilla'),
('IVOIRE','Côte d''Ivoire'),
('COMORES','Comoros'),
('DOMENICA','Dominica'),
('GRANADA','Grenada'),
('MACAU','Macao'),
('MACAU','Lao People''s Democratic Republic'),
('COLUMBIA','Colombia'),
('CAYMAN','Cayman Islands'),
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

def cleanup_prefixes_legacy():
    df = pd.read_excel("ContryCode.xls")
    df = df.drop(columns=['ID'])
    df = df.rename(columns={"PLMNNAME": "carrier_name", "COUNTRY": "country", "COUNTRY_CODE": "CC", "NATIONAL_DESTINATION_CODE":"NDC"})
    df = df[~df['NDC'].str.contains('P', na=False)]
    df['prefix']=''
    df['country'] = df['country'].str.strip()
    df['carrier_name'] = df['carrier_name'].str.strip()

    df['country_upper'] = df['country'].where(df['country'].isnull(), df['country'].str.upper())
    df['carrier_name_upper'] = df['carrier_name'].where(df['carrier_name'].isnull(), df['carrier_name'].str.upper())

    mask = df['carrier_name_upper'].isin(['MEXICO','LATVIA','LITHUANIA'])
    temp = df.loc[mask, 'carrier_name']
    df.loc[mask, 'carrier_name'] = df.loc[mask, 'country']
    df.loc[mask, 'country'] = temp


    for old_val, new_val in country_replacements:
        df.loc[df['country_upper'].str.contains(old_val, na=False, regex=False), 'country'] = new_val

    df = df[['country','carrier_name','CC','NDC', 'prefix']]

    return df

df_prefixes_web=cleanup_prefixes_web()
df_prefixes_legacy=cleanup_prefixes_legacy()
df = df_prefixes_legacy[~df_prefixes_legacy['carrier_name'].isin(df_prefixes_web['carrier_name'])]
print(df.head(20))


l=sorted(df['carrier_name'].dropna().unique().tolist())
print (l)

print (len(df))