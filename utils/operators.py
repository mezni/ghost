import pandas as pd

def process_bands_columns(df):
    """Process and classify Bands columns (2G, GPRS/3G, 4G)."""
    df['2G'] = df['Bands'].str.contains('GSM') | df['Bands'].str.contains('CDMA ')
    df['2G'] = df['2G'].map({True: 'YES', False: ''})
    
    df['GPRS/3G'] = df['Bands'].str.contains('UMTS') | df['Bands'].str.contains('CDMA2000')
    df['GPRS/3G'] = df['GPRS/3G'].map({True: 'YES', False: ''})
    
    df['LTE'] = df['Bands'].str.contains('LTE') 
    df['LTE'] = df['LTE'].map({True: 'YES', False: ''})
    
    return df.drop(columns=["Bands"])

def steer(file_path):
    df = pd.read_excel(file_path, header=1)
    df['Pays'] = df['Pays'].fillna(method='ffill')
    df = df.rename(columns={'VPMN Public Code': 'VLMNCode'})
    df = df.rename(columns={'MCC': 'MNC', 'MNC': 'MCC'})
    return df


def mccmnc(file_path):
    df = pd.read_csv(file_path,sep=';')
    df = df[df['TADIG'].notnull() | df['Bands'].notnull()]
    df=process_bands_columns(df)
    df = df[~((df['2G'].isnull() | (df['2G'] == '')) &
          (df['GPRS/3G'].isnull() | (df['GPRS/3G'] == '')) &
          (df['LTE'].isnull() | (df['LTE'] == '')))]
    df = df.rename(columns={'TADIG': 'PLMNCode'})
    df['Brand'] = df['Brand'].fillna(df['Operator'])


    return df



def merge(df_mccmnc, df_steer): 
    df = pd.merge(df_mccmnc, df_steer, how='outer', left_on='PLMNCode', right_on='VLMNCode')
    df = df.rename(columns={'MCC_x': 'MCC', 'MNC_x': 'MNC',  '2G_x': '2G',  'GPRS/3G_x': 'GPRS/3G', 'LTE_x': 'LTE'})
    df['Source'] = 'system'
    df['MCC'] = df['MCC'].fillna(df['MCC_y'])
    df['MNC'] = df['MNC'].fillna(df['MNC_y'])
    df['PLMNCode'] = df['PLMNCode'].fillna(df['VLMNCode'])
    df['2G'] = df['2G'].fillna(df['2G_y'])
    df['GPRS/3G'] = df['GPRS/3G'].fillna(df['GPRS/3G_y'])
    df['LTE'] = df['LTE'].fillna(df['LTE_y'])
    df['Brand'] = df['Brand'].fillna(df['Networks'])
    df['Country'] = df['Country'].fillna(df['Pays'])
    df['NetworkName'] = df['Brand']
    df['NetworkName'] = df['NetworkName'].fillna(df['Networks'])
    df['PLMN'] = df['PLMN'].fillna(
        df['MCC'].astype('Int64').astype(str) + df['MNC'].astype('Int64').astype(str).str.zfill(2)
    )
    df['NetworkId'] = df['NetworkName'].str.lower().str.replace(' ', '_', regex=False)+'_'+df['Country'].str.lower().str.replace(' ', '_', regex=False)

    df_operators = df[['MCC', 'MNC', 'PLMN', 'Region', 'Country', 'ISO', 'Operator', 'Brand','PLMNCode', '2G', 'GPRS/3G', 'LTE', 'NetworkId', 'NetworkName','Source']]
    df_steering_cfg = df[['NetworkId', 'NetworkName', 'Rate', 'Routage', 'Source']]

    return df_operators, df_steering_cfg

df_steer=steer("INPUT/steering_plan.xlsx")
df_mccmnc=mccmnc("INPUT/mcc-mnc.csv")

df_operators, df_steering_cfg=merge(df_mccmnc, df_steer)

df_operators.to_csv('OUTPUT/operators.csv', index=False)
df_steering_cfg.to_csv('OUTPUT/steering_cfg.csv', index=False)