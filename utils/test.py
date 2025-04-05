import pandas as pd 



def process_bands_columns(df):
    """Process and classify Bands columns (2G, GPRS/3G, 4G)."""
    df['2G'] = df['Bands'].str.contains('GSM') | df['Bands'].str.contains('CDMA ')
    df['2G'] = df['2G'].map({True: 'X', False: ''})
    
    df['GPRS/3G'] = df['Bands'].str.contains('UMTS') | df['Bands'].str.contains('CDMA2000')
    df['GPRS/3G'] = df['GPRS/3G'].map({True: 'X', False: ''})
    
    df['4G'] = df['Bands'].str.contains('LTE') 
    df['4G'] = df['4G'].map({True: 'X', False: ''})
    
    return df.drop(columns=["Bands"])


df = pd.read_csv('INPUT/mcc-mnc.csv',sep=';')
df = df[df['TADIG'].notnull() & df['Bands'].notnull()]

df=process_bands_columns(df)
df['Brand'] = df['Brand'].fillna(df['Operator'])
df['NetworkId'] = df['Brand'].str.lower().str.replace(' ', '_', regex=False)+'_'+df['Country'].str.lower().str.replace(' ', '_', regex=False)

#df.to_csv('OUTPUT/operators.csv', index=False)


df_steer = pd.read_excel("INPUT/steering_plan.xlsx", header=1)
result = pd.merge(df, df_steer, how='right', left_on='TADIG', right_on='VPMN Public Code')
result = result.rename(columns={'MCC_x': 'MCC', 'MNC_x': 'MNC',  '2G_x': '2G',  'GPRS/3G_x': 'GPRS/3G', 'Networks': 'NetworkName'})

result['MCC'] = result['MCC'].apply(lambda x: int(x) if pd.notna(x) else x)
result['MNC'] = result['MNC'].apply(lambda x: int(x) if pd.notna(x) else x)
result['PLMN'] = result['PLMN'].apply(lambda x: int(x) if pd.notna(x) else x)
#print (result.head())

result = result[['MCC','MNC','PLMN','Region','Country','ISO','Operator','Brand','TADIG','2G','GPRS/3G','4G','NetworkId', 'NetworkName','VPMN Public Code']]
print (result.head())

result.to_csv('OUTPUT/operators_new.csv', index=False)