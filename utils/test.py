import pandas as pd
import json

df = pd.read_excel("ContryCode.xls")
print (len(df))

df = df.drop(columns=['ID'])
df = df.rename(columns={"PLMNNAME": "carrier_name", "COUNTRY": "country", "COUNTRY_CODE": "CC", "NATIONAL_DESTINATION_CODE":"NDC"})
df['CC'] = df['CC'].astype(str)

df = df[~df['NDC'].str.contains('P', na=False)]

df['carrier_name_upper'] = df['carrier_name'].str.upper()

df.loc[df['carrier_name_upper'].str.contains('ORANGE', na=False), 'carrier_name'] = 'Orange'
df.loc[df['carrier_name_upper'].str.contains('MTN', na=False), 'carrier_name'] = 'MTN'
df.loc[df['carrier_name_upper'].str.contains('CELTEL', na=False), 'carrier_name'] = 'Celtel'
df.loc[df['carrier_name_upper'].str.contains('BOUYGUTEL', na=False), 'carrier_name'] = 'Bouygues'
df.loc[df['carrier_name_upper'].str.contains('O2', na=False), 'carrier_name'] = 'O2'
df.loc[df['carrier_name_upper'].str.contains('T-MOBILE', na=False), 'carrier_name'] = 'T-Mobile'
df.loc[df['carrier_name_upper'].str.contains('VODAFONE', na=False), 'carrier_name'] = 'Vodafone'
df.loc[df['carrier_name_upper'].str.contains('KPN', na=False), 'carrier_name'] = 'KPN'
df.loc[df['carrier_name_upper'].str.contains('TELENOR', na=False), 'carrier_name'] = 'Telenor'
df.loc[df['carrier_name_upper'].str.contains('GLOBE', na=False), 'carrier_name'] = 'GLOBE'
df.loc[df['carrier_name_upper'].str.contains('MEGAFON', na=False), 'carrier_name'] = 'MegaFon'
df.loc[df['carrier_name_upper'].str.contains('TELE2', na=False), 'carrier_name'] = 'Tele2'
df.loc[df['carrier_name_upper'].str.contains('VODACOM', na=False), 'carrier_name'] = 'Vodacom'
df.loc[df['carrier_name_upper'].str.contains('AT&T', na=False), 'carrier_name'] = 'AT&T'
df.loc[df['carrier_name_upper'].str.contains('AT&T', na=False), 'carrier_name'] = 'AT&T'
df.loc[df['carrier_name_upper'].str.contains('ETTISALET', na=False), 'carrier_name'] = 'Etisalet'
df.loc[df['carrier_name_upper'].str.contains('ETISALET', na=False), 'carrier_name'] = 'Etisalet'
df.loc[df['carrier_name_upper'].str.contains('VIVA', na=False), 'carrier_name'] = 'Viva'
df.loc[df['carrier_name_upper'].str.contains('TATA', na=False), 'carrier_name'] = 'Tata'
df.loc[df['carrier_name_upper'].str.contains('TMOBILE', na=False), 'carrier_name'] = 'T-Mobile'
df.loc[df['carrier_name_upper'].str.contains('T-MOBILE', na=False), 'carrier_name'] = 'T-Mobile'
df.loc[df['carrier_name_upper'].str.contains('WATANIYA', na=False), 'carrier_name'] = 'Wataniya'
df.loc[df['carrier_name_upper'].str.contains('IDEA', na=False), 'carrier_name'] = 'Idea'
df.loc[df['carrier_name_upper'].str.contains('AT-T', na=False), 'carrier_name'] = 'AT&T'
df.loc[df['carrier_name_upper'].str.contains('TELUS', na=False), 'carrier_name'] = 'Telus'
df.loc[df['carrier_name_upper'].str.contains('TIGO', na=False), 'carrier_name'] = 'Tigo'
df.loc[df['carrier_name_upper'].str.contains('ZAIN', na=False), 'carrier_name'] = 'Zain'
df.loc[df['carrier_name_upper'].str.contains('DIGICEL', na=False), 'carrier_name'] = 'Digicel'
df.loc[df['carrier_name_upper'].str.contains('AIRCEL', na=False), 'carrier_name'] = 'Aircel'
df.loc[df['carrier_name_upper'].str.contains('CLARO', na=False), 'carrier_name'] = 'Claro'
df.loc[df['carrier_name_upper'].str.contains('NEXTEL', na=False), 'carrier_name'] = 'Nextel'
df.loc[df['carrier_name_upper'].str.contains('BHARTI', na=False), 'carrier_name'] = 'Bharti'
df.loc[df['carrier_name_upper'].str.contains('SURE', na=False), 'carrier_name'] = 'Sure'
df.loc[df['carrier_name_upper'].str.contains('TELEFONICA', na=False), 'carrier_name'] = 'Telefonica'
df.loc[df['carrier_name_upper'].str.contains('VODACOM', na=False), 'carrier_name'] = 'Vodacom'
df.loc[df['carrier_name_upper'].str.contains('CHINAMOBLTD', na=False), 'carrier_name'] = 'China Mobile'
df.loc[df['carrier_name_upper'].str.contains('CHINA MOB', na=False), 'carrier_name'] = 'China Mobile'
df.loc[df['carrier_name_upper'].str.contains('BLUSKY', na=False), 'carrier_name'] = 'Bluesky'
df.loc[df['carrier_name_upper'].str.contains('BLUESKY', na=False), 'carrier_name'] = 'Bluesky'
df.loc[df['carrier_name_upper'].str.contains('ALTAN', na=False), 'carrier_name'] = 'Altan'
df.loc[df['carrier_name_upper'].str.contains('CHINAUNICOM', na=False), 'carrier_name'] = 'China Unicom'
df.loc[df['carrier_name_upper'].str.contains('H3G', na=False), 'carrier_name'] = 'H3G'
df.loc[df['carrier_name_upper'].str.contains('BELL', na=False), 'carrier_name'] = 'BELL'
df.loc[df['carrier_name_upper'].str.contains('HUTCHISON', na=False), 'carrier_name'] = 'Hutchison'
df.loc[df['carrier_name_upper'].str.contains('MOOV', na=False), 'carrier_name'] = 'Moov'
df.loc[df['carrier_name_upper'].str.contains('CLARPURTRICO', na=False), 'carrier_name'] = 'Claro'
df.loc[df['carrier_name_upper'].str.contains('VIETTEL', na=False), 'carrier_name'] = 'Viettel'
df.loc[df['carrier_name_upper'].str.contains('ORANGCARAIBE', na=False), 'carrier_name'] = 'ORANGE'
df.loc[df['carrier_name_upper'].str.contains('MTS', na=False), 'carrier_name'] = 'MTS'
df.loc[df['carrier_name_upper'].str.contains('OOREDOO', na=False), 'carrier_name'] = 'Ooredoo'
df.loc[df['carrier_name_upper'].str.contains('TN TELECOM', na=False), 'carrier_name'] = 'Tunisie Telecom'
df.loc[df['carrier_name_upper'].str.contains('TUNISIANA', na=False), 'carrier_name'] = 'Ooredoo'
df.loc[df['carrier_name_upper'].str.contains('VOIPORAN', na=False), 'carrier_name'] = 'Orange'
df.loc[df['carrier_name_upper'].str.contains('ADSLORAN', na=False), 'carrier_name'] = 'Orange'
df.loc[df['carrier_name_upper'].str.contains('LIBERTIS', na=False), 'carrier_name'] = 'Libertis'
df.loc[df['carrier_name_upper'].str.contains('AFRICELL', na=False), 'carrier_name'] = 'Africell'
df.loc[df['carrier_name_upper'].str.contains('SCANCOM', na=False), 'carrier_name'] = 'Scancom'
df.loc[df['carrier_name_upper'].str.contains('DIGICLGUYANA', na=False), 'carrier_name'] = 'Digicel'
df.loc[df['carrier_name_upper'].str.contains('TELECEL', na=False), 'carrier_name'] = 'Telecel'
df.loc[df['carrier_name_upper'].str.contains('OPTIMUMTELEC', na=False), 'carrier_name'] = 'Djezzy'






#df = df[df['carrier_name_upper'].str.contains('TN TELECOM', na=False)]
df['country'] = df['country'].str.strip()
df['carrier_name'] = df['carrier_name'].str.strip()
df = df.drop_duplicates()
df = df [['country', 'carrier_name', 'CC','NDC']]
df = df.sort_values(by=['country', 'carrier_name'])

df.to_csv('prefixes.csv', index=False)
print (len(df))
print (df.head(10))
