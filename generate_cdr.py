import time
import random
from datetime import datetime, timedelta

def generate_cdr(call_id):
    cdr = {}
    # Start time is from now minus a period of time within the last hour
    start_time = datetime.now() - timedelta(hours=1) + timedelta(seconds=random.randint(0, 3600))
    duration = random.randint(1, 3600)  # Duration between 1 second and 1 hour
    end_time = start_time + timedelta(seconds=duration)  # Calculate end time based on duration
    
    # Fill the CDR dictionary with the required fields
    cdr['call_id'] = call_id
    cdr['calling_number'] = f"216{random.randint(50, 55)}{random.randint(100000, 999999)}"    
    cdr['called_number'] = f"216{random.randint(30, 77)}{random.randint(100000, 999999)}" 
    cdr['start_time'] = start_time.strftime('%d/%m/%Y %H:%M:%S') 
    cdr['end_time'] = end_time.strftime('%d/%m/%Y %H:%M:%S') 
    cdr['duration'] = duration  # use the previously calculated duration
    cdr['call_type'] = random.choice(['OUTGOING', 'INCOMING'])   

    return cdr

# Prepare to write CDRs to a CSV file
unix_timestamp = int(datetime.now().timestamp())
call_id = int(str(random.randint(100, 999)) + str(unix_timestamp)[4:])

number_of_file=4
cdr_per_file=10000
for _ in range(number_of_file):
    num_records = 0  
    filename = f"CDR{datetime.now().strftime('%Y%m%d%H%M%S')}.csv"  
    with open(filename, 'w') as file:
        file.write("# File Header\n")
        file.write(f"HDR,1,{datetime.now().strftime('%Y%m%d')},{datetime.now().strftime('%H%M%S')},CDR\n")
        file.write("# CDR Records\n")
        
        for _ in range(cdr_per_file):  # Generate 10 CDRs
            cdr_record = generate_cdr(call_id)
            # Write the CDR record to file in CSV format
            file.write(f"{cdr_record['call_id']},"
                    f"{cdr_record['calling_number']},"
                    f"{cdr_record['called_number']},"
                    f"{cdr_record['start_time']},"
                    f"{cdr_record['end_time']},"
                    f"{cdr_record['duration']},"
                    f"{cdr_record['call_type']}\n")
#            print(cdr_record)  # Print the generated CDR record to console
            call_id += 1
            num_records += 1  # Increment the number of records

        file.write("# File Trailer\n")
        file.write(f"TRL,{num_records},{datetime.now().strftime('%Y%m%d')},{datetime.now().strftime('%H%M%S')},CDR\n")
        file.write("# Checksum\n")
        file.write(f"CHECKSUM:{random.randint(10000000, 99999999)}\n")
    time.sleep(1)
    print(f"CDR records written to {filename}")