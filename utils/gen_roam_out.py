import random
from datetime import datetime
import pandas as pd

start_date = datetime(2025, 4, 1)
end_date = datetime(2025, 4, 11)
WORK_DIR="/home/dali/WORK/DATA/"

dates = pd.date_range(start=start_date, end=end_date).tolist()

df = pd.read_csv('HSS9860_1549_20250314023000.txt')
num_rows = len(df)
lower_bound = num_rows * 0.8  

date_strings = [date.strftime('%Y%m%d') for date in dates]

for date_string in date_strings:
    file_name = "HSS9860_1549_"+date_string+ "000000.txt"

    random_number = random.uniform(lower_bound, num_rows)    
    random_rows = df.sample(n=int(random_number))
    random_rows.to_csv(WORK_DIR+file_name, index=False)

