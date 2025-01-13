#call_id,calling_number,called_number,start_time,end_time,duration,call_type
import random
from datetime import datetime, timedelta



def generate_cdr(call_id):
    cdr= {}
    start_time = datetime.now() - timedelta(hours=1) + timedelta(seconds=random.randint(0, 3600))
    cdr['call_id'] = call_id
    cdr['calling_number'] = f"216{random.randint(50, 55)}{random.randint(100000, 999999)}"    
    cdr['called_number'] = f"216{random.randint(30, 77)}{random.randint(100000, 999999)}" 
    cdr['start_time'] = start_time.strftime('%Y%m%d %H%M%S') 
    cdr['call_duration'] = random.randint(1, 3600)   
    return cdr





unix_timestamp = int(datetime.now().timestamp())
call_id=int(str(random.randint(100, 999))+str(unix_timestamp)[4:])
for _ in range(10): 
    x=generate_cdr(call_id)
    print (x)
    call_id+=1