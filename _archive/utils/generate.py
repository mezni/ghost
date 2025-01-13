import csv
import random
import datetime

def generate_cdr():
    caller_number = ''.join(str(random.randint(0, 9)) for _ in range(10))
    callee_number = ''.join(str(random.randint(0, 9)) for _ in range(10))
    call_duration = random.randint(1, 3600)
    timestamp = datetime.datetime.utcnow() - datetime.timedelta(seconds=random.randint(0, 86400))
    return [caller_number, callee_number, call_duration, timestamp.strftime('%Y-%m-%d %H:%M:%S')]


file_name='example.csv'
with open(file_name, 'w', newline='') as csvfile:
    writer = csv.writer(csvfile)
    writer.writerow(["Caller Number", "Callee Number", "Call Duration", "Timestamp"])
    for _ in range(10000):
        writer.writerow(generate_cdr())