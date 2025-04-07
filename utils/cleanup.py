import pandas as pd
import json

carrier_replacements = [
('ORANGE','Orange'),
('MTN','MTN'),
('CELTEL','Celtel'),
('BOUYGUTEL','Bouygues'),
('O2','O2'),
('T-MOBILE','T-Mobile'),
('VODAFONE','Vodafone'),
('KPN','KPN'),
('TELENOR','Telenor'),
('GLOBE','GLOBE'),
('MEGAFON','MegaFon'),
('TELE2','Tele2'),
('VODACOM','Vodacom'),
('AT&T','AT&T'),
('AT&T','AT&T'),
('ETTISALET','Etisalet'),
('ETISALET','Etisalet'),
('VIVA','Viva'),
('TATA','Tata'),
('TMOBILE','T-Mobile'),
('T-MOBILE','T-Mobile'),
('WATANIYA','Wataniya'),
('IDEA','Idea'),
('AT-T','AT&T'),
('TELUS','Telus'),
('TIGO','Tigo'),
('ZAIN','Zain'),
('DIGICEL','Digicel'),
('AIRCEL','Aircel'),
('CLARO','Claro'),
('NEXTEL','Nextel'),
('BHARTI','Bharti'),
('SURE','Sure'),
('TELEFONICA','Telefonica'),
('VODACOM','Vodacom'),
('CHINAMOBLTD','China Mobile'),
('CHINA MOB','China Mobile'),
('BLUSKY','Bluesky'),
('BLUESKY','Bluesky'),
('ALTAN','Altan'),
('CHINAUNICOM','China Unicom'),
('H3G','H3G'),
('BELL','BELL'),
('HUTCHISON','Hutchison'),
('MOOV','Moov'),
('CLARPURTRICO','Claro'),
('VIETTEL','Viettel'),
('ORANGCARAIBE','ORANGE'),
('MTS','MTS'),
('OOREDOO','Ooredoo'),
('TN TELECOM','Tunisie Telecom'),
('TUNISIANA','Ooredoo'),
('VOIPORAN','Orange'),
('ADSLORAN','Orange'),
('LIBERTIS','Libertis'),
('AFRICELL','Africell'),
('SCANCOM','Scancom'),
('DIGICLGUYANA','Digicel'),
('TELECEL','Telecel'),
('OPTIMUMTELEC','Djezzy'),
('MOBILIS','Mobilis'),
('MOVITEL','Movitel'),
('WATANYAPALES','Wataniya'),
('QTEL QUATA','Qtel'),
('ITISALET','Etisalet'),
('AIRTL','Airtel'),
('AIRTEL','Airtel'),
('SKYUK','Sky'),
('A1TELEKO','A1 Telekom'),
('A1TELEKOM','A1 Telekom'),
#('?LANDS','Alands'),
('AZERFON','Azerfon'),
('BEMOBIL','Bemobile'),
('CWWI','CWWI'),
('CABLE&WIRE','Cable&Wire'),
('ELISA','Elisa'),
('GLOMOB','Glo Mobile'),
('MTXCONNECT','MTXConnect'),
('MATELL','Matell'),
('MOBIMIL','Orange'),
('ORANGDOM','Orange'),
('ORANGMAU','Orange'),
#('TIM (','TIM'),
('TRUEMOV','True Move'),
('TRUEMOVE','True Move'),
('VODAF.','Vodafone'),
('DU UAE.','Du'),
]

# Carrier replacements to normalize names
carrier_web_replacements = [
    ('various ', ''), ('Various ', ''), ('where A is', ''), ('OPERATORS', ''),
    ('Full MVNO', ''), ('/', ''), ('Mainly', ''), ('operator reserved', ''),
    ('All carriers', ''), ('Any carrier', ''), ('O2', 'O2'), ('Other mobile networks', ''),
    ('BeeLine', 'Beeline'), ('Sunrise', 'Sunrise'), ('Vodafone', 'Vodafone'), ('Orange', 'Orange'),
    ('WIND', 'WIND'), ('Wataniya', 'Wataniya'), ('Zain', 'Zain'), ('T-Mobile', 'T-Mobile'),
    ('Tele2', 'Tele2'), ('TIM', 'TIM'), ('mobilkom', 'mobilkom'), ('one.vip', 'one.vip'),
    ('eir/Meteor', 'eir/Meteor'), ('YESSSS!', 'YESSSS!'), ('Zapp Mobile', 'Zapp Mobile'),
    ('Tunisia Tuntel Mobile', 'Tunisie Telecom'), ('Tunisia Mobile Orascom', 'Ooredoo'),
    ('Tesco', 'Tesco'), ('Three', 'Three'), ('Telenor', 'Telenor'), ('TCell', 'TCell'),
    ('Telecel', 'Telecel'), ('Skytel', 'Skytel'), ('Skylink', 'Skylink'),
    ('Smart Comm', 'Smart Communications'), ('UCell', 'UCell'), ('Swisscom', 'Swisscom'),
    ('one-.vip', 'one.vip'), ('AT&T', 'AT&T'), ('airtel', 'Airtel'), ('Comcel', 'Comcel'),
    ('LaiLai', 'LaiLai'), ('Lycamobile', 'Lycamobile'), ('MTN', 'MTN'), ('Mobilis', 'Mobilis'),
    ('Nedjma', 'Ooredoo'), ('Korek', 'Korek'), ('Mobinil', 'Orange'), ('Ooredoo', 'Ooredoo'),
    ('Orascom', 'Ooredoo'), ('Indosat', 'Indosat'), ('Beeline', 'Beeline'),
    ('H3G (Tre)', 'H3G (Tre)'), ('Globe T', 'Globe Telecom'), ('YESSS!', 'YESSS!'),
    ('Telkomsel', 'Telkomsel'),
]

# Country name corrections
country_replacements = [
    ('BUTSWANA','Botswana'), ('BOSNIA','Bosnia & Herzegovina'), ('IRELAND','Ireland'),
    ('BOLIVIA','Bolivia, Plurinational State Of'), ('TIMOR','East Timor'),
    ('SWITZERL','Switzerland'), ('SALVADOR','El Salvador'), ('SYRIA','Syrian Arab Republic'),
    ('TONGA','Tonga'), ('TANZANIA','Tanzania, United Republic Of'), ('TCHAD','Chad'),
    ('TRINIDAD','Trinidad And Tobago'), ('VENEZUELA','Venezuela, Bolivarian Republic Of'),
    ('VIETNAM','Viet Nam'), ('UK','United Kingdom'), ('ROYAUME-UNI','United Kingdom'),
    ('USA','United States'), ('TUNIS','Tunisia'), ('TUNISIE','Tunisia'),
    ('TAJAKISTAN','Tajikistan'), ('TAIWAN','Taiwan'), ('SWITZELAND','Switzerland'),
    ('SURINAM','Suriname'), ('SUREGUER','Guernsey'), ('SOUTHAFRICA','South Africa'),
    ('SLOVAK','Slovakia'), ('SIERRALEONE','Sierra Leone'), ('REPUBLIQUE CHEQUE','Czech Republic'),
    ('ROAMANIA','Romania'), ('RWANDA','Rwanda'), ('REUNION','Reunion'), ('NORWAY','Norway'),
    ('NLECALE','New Caledonia'), ('NEWGUINEA','Papua New Guinea'),
    ('NEW GUINEA','Papua New Guinea'), ('MYANMA','Myanmar'), ('MOZAMBI','Mozambique'),
    ('INDIA','India'), ('MOLDOVA','Moldova'), ('MOLDAVIA','Moldova'),
    ('MAURITAN','Mauritania'), ('MALTE','Malta'), ('MACEDONIA','Macedonia'),
    ('LIBYE','Libya'), ('INDONESI','Indonesia'), ('IVORYCOAST','Côte d\'Ivoire'),
    ('HONGKONG','Hong Kong'), ('GUYANE','Guyana'), ('GUINEEB','Guinea-Bissau'),
    ('GUINEAB','Guinea-Bissau'), ('GUINEE','Guinea'), ('ETHIOP','Ethiopia'), ('FIJI','Fiji'),
    ('GABON','Gabon'), ('GHABON','Gabon'), ('DEUTSCHLAND','Germany'), ('EGYPT','Egypt'),
    ('EQUATORIALGUINEA','Equatorial Guinea'), ('COSTARICA','Costa Rica'),
    ('IRAN','Iran, Islamic Republic Of'), ('UZBEKISTAN','Uzbekistan'), ('THAILAND','Thailand'),
    ('CHINE','China'), ('COMBODGE','Cambodia'), ('CAMBODGE','Cambodia'),
    ('SOUTH SUDAN','South Sudan'), ('KITTS','Saint Kitts And Nevis'),
    ('SAOTOME','São Tomé and Príncipe'), ('RUSSIA','Russian Federation'),
    ('PALESTINE','Palestine, State of'), ('CENTRAL AFRIC','Central African Republic'),
    ('CAPVERT','Cabo Verde'), ('CAP VERD','Cabo Verde'), ('CAMEROUN','Cameroon'),
    ('AZERBAIJAN','Azerbaijan'), ('BORKINAFASO','Burkina Faso'), ('ANTIGUA','Antigua And Barbuda'),
    ('VIRGIN ISL','Virgin Islands (British)'), ('VIRGINISL','Virgin Islands (British)'),
    ('CANADA','Canada'), ('CONGO RDC','Democratic Republic Of Congo'),
    ('CAICOS','Turks And Caicos Islands'), ('BRUNEI','Brunei Darussalam'),
    ('COOK','Cook Islands'), ('CONGO [DRC','Democratic Republic Of Congo'),
    ('CONGO [REPUBLIC','Republic Of Congo'), ('ANGILLA','Anguilla'),
    ('IVOIRE','Côte d\'Ivoire'), ('COMORES','Comoros'), ('DOMENICA','Dominica'),
    ('GRANADA','Grenada'), ('MACAU','Macao'), ('COLUMBIA','Colombia'),
    ('CAYMAN','Cayman Islands'),
]

def cleanup_prefixes_web():
    with open('INPUT/prefixes.json', 'r') as f:
        data = json.load(f)

    df = pd.DataFrame(data)
    df = df.rename(columns={
        "carrierName": "carrier_name",
        "countryName": "country",
        "callingCode": "CC",
        "mobilePrefix": "NDC",
        "fullCode": "prefix"
    })

    # Strip unwanted characters
    df['carrier_name'] = df['carrier_name'].str.replace(r'\(.*?\)', '', regex=True)
    for col in ['CC', 'NDC', 'prefix']:
        df[col] = df[col].str.replace('x', '', regex=False).str.replace('+', '', regex=False)

    # Cleanup special country cases
    df['country'] = df['country'].str.strip()
    df['carrier_name'] = df['carrier_name'].str.strip()
    df.loc[df['country'].str.contains('Sahrawi Arab Democratic Republic|Western Sahara', na=False), 'country'] = 'Morocco'

    for old_val, new_val in carrier_web_replacements:
        df.loc[df['carrier_name'].str.contains(old_val, na=False, regex=False), 'carrier_name'] = new_val

    return df[['country', 'carrier_name', 'CC', 'NDC', 'prefix']].sort_values(by=['country', 'carrier_name'])

def cleanup_prefixes_legacy():
    df = pd.read_excel("INPUT/ContryCode.xls")
    df = df.drop(columns=['ID'])
    df = df.rename(columns={
        "PLMNNAME": "carrier_name",
        "COUNTRY": "country",
        "COUNTRY_CODE": "CC",
        "NATIONAL_DESTINATION_CODE": "NDC"
    })

    df = df[~df['NDC'].str.contains('P', na=False)]
    df['CC'] = df['CC'].apply(lambda x: str(int(x)) if pd.notnull(x) else x)
    df['prefix'] = df['CC'].fillna('').astype(str) + df['NDC'].fillna('').astype(str)

    df['country'] = df['country'].str.strip()
    df['carrier_name'] = df['carrier_name'].str.strip()

    df['country_upper'] = df['country'].str.upper()
    df['carrier_name_upper'] = df['carrier_name'].str.upper()

    # Correct countries that were misclassified as carriers
    mask = df['carrier_name_upper'].isin(['MEXICO', 'LATVIA', 'LITHUANIA'])
    temp = df.loc[mask, 'carrier_name']
    df.loc[mask, 'carrier_name'] = df.loc[mask, 'country']
    df.loc[mask, 'country'] = temp

    for old_val, new_val in country_replacements:
        df.loc[df['country_upper'].str.contains(old_val, na=False, regex=False), 'country'] = new_val

    for old_val, new_val in carrier_replacements:
        df.loc[df['carrier_name_upper'].str.contains(old_val, na=False, regex=False), 'carrier_name'] = new_val

    return df[['country', 'carrier_name', 'CC', 'NDC', 'prefix']]


def process_bands_columns(df):
    """Process and classify Bands columns (2G, GPRS/3G, 4G)."""
    df['2G'] = df['Bands'].str.contains('GSM') | df['Bands'].str.contains('CDMA ')
    df['2G'] = df['2G'].map({True: 'YES', False: ''})
    
    df['GPRS/3G'] = df['Bands'].str.contains('UMTS') | df['Bands'].str.contains('CDMA2000')
    df['GPRS/3G'] = df['GPRS/3G'].map({True: 'YES', False: ''})
    
    df['LTE'] = df['Bands'].str.contains('LTE') 
    df['LTE'] = df['LTE'].map({True: 'YES', False: ''})
    
    return df.drop(columns=["Bands"])

def cleanup_mccmnc_web():
    df = pd.read_csv('INPUT/mcc-mnc.csv',sep=';')
    df = df[df['TADIG'].notnull() & df['Bands'].notnull()]
    df=process_bands_columns(df)
    df['Brand'] = df['Brand'].fillna(df['Operator'])
    
    return df

def cleanup_mccmnc_legacy():
    df = pd.read_excel("INPUT/steering_plan.xlsx", header=1)
    return df



df_prefixes_web = cleanup_prefixes_web()
df_prefixes_legacy = cleanup_prefixes_legacy()

#carrier_list = sorted(df_prefixes_web['carrier_name'].dropna().unique().tolist(), key=len, reverse=True)
#carrier_list_legacy = sorted(df_prefixes_legacy['carrier_name'].dropna().unique().tolist(), key=len, reverse=True)

# Perform outer join on 'country' and 'carrier_name'
df_combined = pd.merge(df_prefixes_web, df_prefixes_legacy, on=['country', 'carrier_name'], how='outer', suffixes=('_web', '_legacy'))

# Optionally, you can sort the result by country and carrier_name for better readability
df_combined['CC_web'] = df_combined['CC_web'].fillna(df_combined['CC_legacy'])
df_combined['NDC_web'] = df_combined['NDC_web'].fillna(df_combined['NDC_legacy'])
df_combined['prefix_web'] = df_combined['prefix_web'].fillna(df_combined['prefix_legacy'])

# Optionally, you can sort the result by country and carrier_name for better readability
df_combined = df_combined.sort_values(by=['country', 'carrier_name'])

df_combined = df_combined.rename(columns={
        "CC_web": "CC",
        "countryName": "country",
        "NDC_web": "NDC",
        "prefix_web": "prefix"
    })

df_combined['carrier_name_upper'] = df_combined['carrier_name'].str.upper()
for old_val, new_val in carrier_replacements:
    df_combined.loc[df_combined['carrier_name_upper'].str.contains(old_val, na=False, regex=False), 'carrier_name'] = new_val

df = df_combined[['country', 'carrier_name', 'CC', 'NDC', 'prefix']]
df = df.drop_duplicates(subset=['country','prefix'], keep='first')
df = df.sort_values(by=['country','prefix'])



df.to_csv('OUTPUT/prefixes.csv',index=False)


df_mccmnc_web= cleanup_mccmnc_web()
df_mccmnc_legacy= cleanup_mccmnc_legacy()
df_combined = pd.merge(df_mccmnc_legacy, df_mccmnc_web, how='outer', left_on='VPMN Public Code', right_on='TADIG')

df_combined['VPMN Public Code'] = df_combined['VPMN Public Code'].fillna(df_combined['TADIG'])
df_combined['Pays'] = df_combined['Pays'].fillna(df_combined['Country'])
df_combined['Networks'] = df_combined['Networks'].fillna(df_combined['Brand'])
df_combined = df_combined.drop(columns=['Brand','TADIG','2G_y','GPRS/3G_y','LTE_y','MCC_y','MNC_y','Pays'])
df_combined = df_combined.rename(columns={
        "MNC_x": "MNC",
        "MCC_x": "MCC",
        "2G_x": "2G",
        "GPRS/3G_x": "GPRS/3G",
        "LTE_x": "LTE",
#        'Networks':'Operator'
    })
df_combined['PLMN'] = df_combined['PLMN'].apply(lambda x: str(int(x)) if pd.notnull(x) else x)


df_combined['Networks_upper'] = df_combined['Networks'].str.upper()
for old_val, new_val in carrier_replacements:
    df_combined.loc[df_combined['Networks_upper'].str.contains(old_val, na=False, regex=False), 'Networks'] = new_val

df = df_combined[[ 'Country','Region', 'ISO']]
df = df.drop_duplicates(subset=['Country','Region', 'ISO'], keep='first')
df.to_csv('OUTPUT/countries.csv', index=False)


df = df_combined[['VPMN Public Code','Networks', 'Rate',  'Routage']]
df = df.drop_duplicates(subset=['VPMN Public Code'], keep='first')
df.to_csv('OUTPUT/steering.csv', index=False)

df = df_combined[['VPMN Public Code','Networks', 'Rate',  'MNC',  'MCC',    '2G', 'GPRS/3G',  'LTE', 'PLMN', 'Country']]
df = df.drop_duplicates(subset=['VPMN Public Code'], keep='first')
df.to_csv('OUTPUT/networks.csv', index=False)
