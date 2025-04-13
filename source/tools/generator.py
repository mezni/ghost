import random
from datetime import datetime, timedelta
import pandas as pd
from pathlib import Path

# Use today's date and compute the date range
end_date = datetime.today()
start_date = end_date - timedelta(days=10)

WORK_DIR = "/app/INPUT"
Path(WORK_DIR).mkdir(parents=True, exist_ok=True)

source_file = 'roam_out_ref.txt'

# Read source file
try:
    df = pd.read_csv(source_file)
except FileNotFoundError:
    print(f"Source file not found: {source_file}")
    exit(1)

num_rows = len(df)
lower_bound = int(num_rows * 0.8)

date_strings = [date.strftime('%Y%m%d') for date in pd.date_range(start=start_date, end=end_date)]

for date_string in date_strings:
    file_name = f"HSS9860_1549_{date_string}000000.txt"
    sample_size = random.randint(lower_bound, num_rows)
    random_rows = df.sample(n=sample_size)
    output_path = Path(WORK_DIR) / file_name
    random_rows.to_csv(output_path, index=False)
    print(f"Generated: {output_path}")
