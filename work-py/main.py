import sqlite3

def create_database(db_name, sql_script):
    try:
        with sqlite3.connect(db_name) as conn:
            cursor = conn.cursor()
            with open(sql_script, 'r') as f:
                sql = f.read()
            cursor.executescript(sql)
            conn.commit()
        return True
    except sqlite3.Error as e:
        print(f"Error creating database: {e}")
        return False


db_name = 'test.db'
sql_script = 'schema.sql'
#create_database(db_name, sql_script)


import pandas as pd
from datetime import datetime, timedelta

# Define the start and end dates
start_date = datetime(2020, 1, 1)
end_date = datetime(2025, 12, 31)

# Create a date range
date_range = pd.date_range(start=start_date, end=end_date)

# Create a DataFrame
df = pd.DataFrame(date_range, columns=['date_key'])

# Add additional columns
df['year'] = df['date_key'].dt.year
df['quarter'] = df['date_key'].dt.quarter
df['month'] = df['date_key'].dt.month
df['month_name'] = df['date_key'].dt.strftime('%B')
df['day'] = df['date_key'].dt.day
df['day_of_week'] = df['date_key'].dt.dayofweek
df['day_name'] = df['date_key'].dt.strftime('%A')
df['is_weekend'] = df['date_key'].dt.dayofweek.isin([5, 6])
df['is_holiday'] = False  # You can add holiday logic here

# Print the DataFrame
print(df.head())

# Save to SQLite database
import sqlite3
conn = sqlite3.connect('calendar_dim.db')
df.to_sql('calendar_dim', conn, if_exists='replace', index=False)
conn.close()