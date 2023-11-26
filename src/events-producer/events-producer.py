import configparser
import yaml
import os, time, json, requests
from datetime import datetime, timedelta
from confluent_kafka import Producer


def read_yaml(file_name):
    with open(file_name, "r") as stream:
        try:
            return yaml.safe_load(stream)
        except yaml.YAMLError as exc:
            print(exc)


def get_events(events_start_date, interval_mins, trx_count):
    headers = {"Content-type": "application/json", "Accept": "text/plain"}
    data = {
        "interval_start": events_start_date,
        "interval_mins": interval_mins,
        "trx_count": trx_count,
    }

    r = requests.post(url, data=json.dumps(data), headers=headers)
    status = r.status_code
    result = r.json()
    if status == 200:
        return result["Records"]
    else:
        return []


def acked(err, msg):
    if err is not None:
        print("Failed to deliver message: %s: %s" % (str(msg), str(err)))


#    else:
#        print("Message produced: %s" % (str(msg)))


def generate_next_date(events_start_date, interval_mins, interval_multiplier):
    next_start_date_time = datetime.strptime(
        events_start_date, "%d/%m/%Y %H:%M:%S"
    ) + timedelta(minutes=interval_mins * interval_multiplier)
    next_start_date = next_start_date_time.strftime("%d/%m/%Y %H:%M:%S")
    return next_start_date


config = configparser.ConfigParser()
config.read("config.ini")

config_file = config.get("config", "config_file")
events_start_date = config.get("config", "events_start_date")
interval_mins = config.get("config", "interval_mins")
trx_count = config.get("config", "trx_count")

service_conf = read_yaml(config_file)
bootstrap_servers = service_conf["kafka"]["servers"]
topic = service_conf["kafka"]["topic"]
url = service_conf["kafka"]["url"]

conf = {bootstrap_servers}

producer = Producer(**conf)

f = open("lock.lck", "w")
f.close()

i = 0
while True:
    next_start_date = generate_next_date(events_start_date, interval_mins, i)
    for j in range(10000):
        if os.path.exists("lock.lck"):
            events = get_events(next_start_date, interval_mins, trx_count)
            for event in events:
                producer.produce(
                    topic, json.dumps(event).encode("utf-8"), callback=acked
                )
            producer.poll(1)
            time.sleep(1)
        else:
            break
    i = i + 1
