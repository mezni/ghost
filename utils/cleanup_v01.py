import pandas as pd

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

country_replacements = [
('VIET NAM','Vietnam'),
('UK','United Kingdom'),
('ROYAUME-UNI','United Kingdom'),
('USA','United States'),
('TUNIS','Tunisia'),
('TUNISIE','Tunisia'),
('TANZANIA','Tanzania'),
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
('Libye','Libya'),
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
('IVOIRE','Côte d''Ivoire'),
('CHINE','China'),
('CAMBODGE','Cambodia'),
('CENTRAL AFRIC','Central African Republic'),
('CAPVERT','Cabo Verde'),
('CAP VERD','Cabo Verde'),
('CAMEROUN','Cameroon'),
('AZERBAIJAN','Azerbaijan'),
('BORKINAFASO','Burkina Faso'),
('ANTIGUA','Antigua And Barbuda'),
('VIRGIN ISL','British Virgin Islands'),
('VIRGINISL','British Virgin Islands'),
('CANADA','Canada'),
('CONGO RDC','Democratic Republic Of Congo'),
('CAICOS','Turks And Caicos Islands'),
('BRUNEI','Brunei Darussalam'),
('COOK','Cook Islands'),
('CONGO [DRC', 'Democratic Republic Of Congo'),
('CONGO [REPUBLIC', 'Republic Of Congo'),
('ANGILLA', 'Anguilla'),
('BOLIV', 'Bolivia'),
]

df = pd.read_excel("ContryCode.xls")
df = df.drop(columns=['ID'])
df = df.rename(columns={"PLMNNAME": "carrier_name", "COUNTRY": "country", "COUNTRY_CODE": "CC", "NATIONAL_DESTINATION_CODE":"NDC"})
df = df[~df['NDC'].str.contains('P', na=False)]
df['country'] = df['country'].str.strip()
df['carrier_name'] = df['carrier_name'].str.strip()
df['country_upper'] = df['country'].where(df['country'].isnull(), df['country'].str.upper())
df['carrier_name_upper'] = df['carrier_name'].where(df['carrier_name'].isnull(), df['carrier_name'].str.upper())

mask = df['carrier_name_upper'].isin(['MEXICO','LATVIA','LITHUANIA'])
temp = df.loc[mask, 'carrier_name']
df.loc[mask, 'carrier_name'] = df.loc[mask, 'country']
df.loc[mask, 'country'] = temp


for old_val, new_val in carrier_replacements:
    df.loc[df['carrier_name_upper'].str.contains(old_val, na=False, regex=False), 'carrier_name'] = new_val


for old_val, new_val in country_replacements:
    df.loc[df['country_upper'].str.contains(old_val, na=False, regex=False), 'country'] = new_val


df = df.sort_values(by=['country','carrier_name'])
df = df[['country','carrier_name','CC','NDC']]
df['CC'] = df['CC'].apply(lambda x: str(int(x)) if pd.notnull(x) else x)
df['prefix'] = df['CC'].fillna('').astype(str) + df['NDC'].fillna('').astype(str)

df_legacy=df



#l=sorted(df['country'].dropna().unique().tolist())
#print (l)

#df.to_csv('prefixes.csv', index=False)


import pandas as pd
import json

# Read the JSON file
with open('prefixes.json', 'r') as f:
    data = json.load(f)

# Convert to DataFrame
df = pd.DataFrame(data)


df = df.rename(columns={"carrierName": "carrier_name", "countryName": "country", "callingCode": "CC", "mobilePrefix":"NDC", "fullCode":"prefix"})
df['CC'] = df['CC'].str.replace('x', '', regex=False)
df['NDC'] = df['NDC'].str.replace('x', '', regex=False)
df['prefix'] = df['prefix'].str.replace('x', '', regex=False)
df['CC'] = df['CC'].str.replace('+', '', regex=False)
df['NDC'] = df['NDC'].str.replace('+', '', regex=False)
df['prefix'] = df['prefix'].str.replace('+', '', regex=False)
df = df[['country','carrier_name','CC','NDC', 'prefix']]

df_web=df
df = pd.concat([df_web, df_legacy], ignore_index=True)
df = df.sort_values(by=['country','carrier_name'])
print(df.head(20))


l=sorted(df['country'].dropna().unique().tolist())
print (l)

print (len(df))