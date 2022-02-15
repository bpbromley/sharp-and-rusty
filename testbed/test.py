import os
import random
import requests
import time
import uuid

keys = set()

for idx in range(16385):
    keys.add(str(uuid.uuid4()))

urls = ['http://localhost:8000/cache/', 'http://localhost:5153/cache/']

for u in urls:
    print("started seeding " + u)
    lt = time.time()
    for k in keys:
        url = u + k
        payload = "test - " + k
        x = requests.post(url, data = payload)
        y = requests.get(url)
        if(payload != y.text):
            print(payload)
            print(y.text)
            raise Exception("failure retrieving " + payload + " from " + url)
    lft = time.time()
    print("finished seeding " + u)
    print("single-thread seeding of " + u + " took {0}".format(lft - lt))
    print("")

chosenOne = random.choice(list(keys))
for u in urls:
    print("")
    print("Benching " + u)
    os.system("wrk -t4 -c400 -d30s " + u + chosenOne)
