import re
import random
import pandas as pd
from pathlib import Path
from datetime import datetime, timedelta

DAYS_TO_GENERATE=10

WORK_DIR = Path("/app/WORK")
INPUT_DIR = WORK_DIR / "INPUT/ROUT"
file_path = WORK_DIR / 'status.txt'

date_pattern = r'(\d{4}-\d{2}-\d{2})' 

date_start = None


source_file = 'roam_out_ref.txt'

# Read source file
try:
    df = pd.read_csv(source_file)
except FileNotFoundError:
    print(f"Source file not found: {source_file}")
    exit(1)

Path(WORK_DIR).mkdir(parents=True, exist_ok=True)
Path(INPUT_DIR).mkdir(parents=True, exist_ok=True)

try:
    with open(file_path, 'r') as file:
        for line in file:
            match = re.search(date_pattern, line)
            if match:
                date_str = match.group(0)
                date_start = datetime.strptime(date_str, '%Y-%m-%d')
                date_start = date_start + timedelta(days=1)
                break
except FileNotFoundError:
    pass

if date_start is None:
    end_date = datetime.today()
    start_date = end_date - timedelta(days=DAYS_TO_GENERATE)
else:
    end_date = datetime.today()
    start_date = date_start

num_rows = len(df)
lower_bound = int(num_rows * 0.8)

date_strings = [date.strftime('%Y%m%d') for date in pd.date_range(start=start_date, end=end_date)]

for date_string in date_strings:
    file_name = f"HSS9860_1549_{date_string}000000.txt"
    sample_size = random.randint(lower_bound, num_rows)
    random_rows = df.sample(n=sample_size)
    output_path = Path(INPUT_DIR) / file_name
    random_rows.to_csv(output_path, index=False)
    print(f"Generated: {output_path}")

print(f"Writing status file: {file_path.absolute()}")
with open(file_path, 'w') as file:
    file.write(datetime.today().strftime('%Y-%m-%d') + '\n') 