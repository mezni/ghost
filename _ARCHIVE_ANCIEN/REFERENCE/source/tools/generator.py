import re
import os
import random
import pandas as pd
from pathlib import Path
from datetime import datetime, timedelta

# Configuration
DAYS_TO_GENERATE = 10
WORK_DIR = Path("/app/WORK")
#WORK_DIR = Path("./WORK")
INPUT_ROUT_DIR = WORK_DIR / "INPUT/ROUT"
INPUT_RIN_DIR = WORK_DIR / "INPUT/RIN"
OUT_STATUS_FILE = INPUT_ROUT_DIR / 'status.txt'
IN_STATUS_FILE = INPUT_RIN_DIR / 'status.txt'
DATE_PATTERN = r'(\d{4}-\d{2}-\d{2})'
OUT_SOURCE_FILE = 'roam_out_ref.txt'
IN_SOURCE_FILE = "roam_in_ref.txt"

def setup_directories():
    """Ensure all required directories exist"""
    Path(WORK_DIR).mkdir(parents=True, exist_ok=True)
    Path(INPUT_ROUT_DIR).mkdir(parents=True, exist_ok=True)
    Path(INPUT_RIN_DIR).mkdir(parents=True, exist_ok=True)

def generate_dates(last_date):
    """Generate date range to process"""
    end_date = datetime.today()
    start_date = last_date if last_date else end_date - timedelta(days=DAYS_TO_GENERATE)
    return [date.strftime('%Y%m%d') for date in pd.date_range(start=start_date, end=end_date)]

def get_last_processed_date(status_file):
    """Read last processed date from status file"""
    try:
        with open(status_file, 'r') as file:
            for line in file:
                match = re.search(DATE_PATTERN, line)
                if match:
                    date_str = match.group(0)
                    return datetime.strptime(date_str, '%Y-%m-%d') + timedelta(days=1)
    except FileNotFoundError:
        return None

def process_in():
    old_date="250306"
    last_date = get_last_processed_date(IN_STATUS_FILE)
    date_strings = generate_dates(last_date)

    for date_string in date_strings:
        file_name = f"ROAMIN_{date_string}.txt" 
        output_path = INPUT_RIN_DIR / file_name

        variance = random.randint(0, 3)
        random_bool = bool(random.getrandbits(1))

        with open(IN_SOURCE_FILE, 'r') as file:
            lines = file.readlines()
            
        modified_lines = []
        for line in lines:
            if line.startswith('ACT') and 'MSCBC2' in line and old_date in line:
                        # Replace the old date with new date
                line = line.replace(old_date, date_string)


            if line.startswith('4-'):
                parts = line.split()
                if len(parts) >= 3:
                    hlr_addr = parts[0]
                    nsub = parts[1]
                    nsuba = parts[2] 
                    if random_bool:
                        line = line.replace(nsub, str(int(nsub)+variance))
                    else:
                        if int(nsub)-variance >= 0:
                            line = line.replace(nsub, str(int(nsub)-variance))
                        else:
                            line = line

            modified_lines.append(line)

        with open(output_path, 'w') as file:
            file.writelines(modified_lines)    

        os.chmod(output_path, 0o777)  # Set file permissions to 777

        print(f"Generated: {output_path}")


    # Update status file
    print(f"Writing status file: {IN_STATUS_FILE.absolute()}")
    with open(IN_STATUS_FILE, 'w') as file:
        file.write(datetime.today().strftime('%Y-%m-%d') + '\n')

def process_out():

    last_date = get_last_processed_date(OUT_STATUS_FILE)
    date_strings  = generate_dates(last_date)

    try:
        df = pd.read_csv(OUT_SOURCE_FILE)
    except FileNotFoundError:
        print(f"Source file not found: {OUT_SOURCE_FILE}")
        exit(1)

    # Sample and write output files
    num_rows = len(df)
    lower_bound = int(num_rows * 0.8)

    for date_string in date_strings:
        file_name = f"HSS9860_1549_{date_string}000000.txt"
        sample_size = random.randint(lower_bound, num_rows)
        random_rows = df.sample(n=sample_size)
        output_path = INPUT_ROUT_DIR / file_name
        random_rows.to_csv(output_path, index=False)

        os.chmod(output_path, 0o777)  # Set file permissions to 777

        print(f"Generated: {output_path}")

    # Update status file
    print(f"Writing status file: {OUT_STATUS_FILE.absolute()}")
    with open(OUT_STATUS_FILE, 'w') as file:
        file.write(datetime.today().strftime('%Y-%m-%d') + '\n')

def main():
    setup_directories()

    process_out()
    process_in()

main()