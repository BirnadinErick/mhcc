from datetime import datetime
import math
import sqlite3
import random
from faker import Faker
from rich.console import Console
from rich.progress import track
import os

# get db cursor
conn = sqlite3.connect(os.path.join(
    os.path.dirname(__file__), 'src-tauri', 'mhcc.brain'))
cursor = conn.cursor()
# ----------------------------------------------------

# init faker
faker = Faker()
fake_data = {
    'staffs': [],
    'stocks': [],
    'grns': [],
    'visitors': [],
    'sales': [],
    'sales_items': [],
}
# ----------------------------------------------------

# init rich
console = Console()
# ----------------------------------------------------

# bootstrap consts
NUM_STAFFS = 20
NUM_STOCKS = 20300
NUM_VISITORS = 100
NUM_GRNS = 4350
NUM_SALES = 2003
NUM_SALES_ITEMS = 20200
# ----------------------------------------------------

# seed staffs
console.rule("[bold blue]Staffs")

for i in track(range(NUM_STAFFS+1), description="generating staffs..."):
    datum = (
        i,  # staff_id
        faker.name().replace(',', ''),  # name
        faker.unique.name().replace(
            ' ',
            random.choice(['*', '_', '?', '+'])
        ).lower().replace(',', ''),  # uname
        faker.name().replace(
            ' ', '-').lower()[::-2].replace(',', ''),  # passwd
        random.randint(1, 3),  # role
        math.ceil(
            faker.past_datetime(start_date='-1y').timestamp()
        )  # date_enrolled
    )
    fake_data['staffs'].append(datum)

# console.log(fake_data['staffs'][:100])
console.log("Done seeding staffs!")
# ----------------------------------------------------------

print("\n")

# seed stocks
console.rule("[bold blue]Stocks")

for i in track(range(NUM_STOCKS+1), description="seeding stocks..."):

    datum = (
        i,  # stock_id
        ''.join(faker.name().lower().replace(
            ' ', '-')).replace(',', ''),  # name
        faker.company().replace(',', ''),  # dispenser
        round(random.uniform(19.0, 2003.0), 2),  # uprice
        random.randint(123, 1234),  # quantity
        math.ceil(
            faker.future_datetime(end_date='+2y').timestamp()
        ),  # date_expiry
        random.randint(1, NUM_STAFFS)  # staff_stocked
    )

    fake_data['stocks'].append(datum)

# console.log(fake_data['stocks'][:100])
console.log("Done seeding stocks!")
# ----------------------------------------------------------

print("\n")

# seed grns
console.rule("[bold blue]Good Returns Note")

for i in track(range(NUM_GRNS+1), description="seeding grns..."):
    datum = (
        i,  # grn_id
        math.ceil(datetime.now().timestamp()),  # date_returned
        random.choice([staff[0]
                      for staff in fake_data['staffs']])  # staff_returned
    )
    fake_data['grns'].append(datum)

# console.log(fake_data['grns'][:100])
console.log("Done seeding grns!")
# ----------------------------------------------------

print("\n")

# seed visitors
console.rule("[bold blue]Visitors")

for i in track(range(NUM_VISITORS+1), description="seeding visitors..."):
    dob = faker.date_between(start_date='-30y', end_date='-4y')
    datum = (
        i,  # v_id
        faker.name().replace(',', ''),  # name
        faker.address().replace('\n', ' ').replace(',', ';'),  # address
        faker.phone_number(),  # tpno
        datetime(dob.year, dob.month, dob.day).timestamp(),  # dob
        faker.ssn(),  # nic
    )

    fake_data['visitors'].append(datum)

# console.log(fake_data['visitors'][:100])
console.log("Done seeding visitors!")
# ----------------------------------------------------

print("\n")

# seed sales
console.rule("[bold blue]Sales")

for i in track(range(NUM_STOCKS+1), description="seeding sales..."):

    date_sold = faker.date_between(start_date="-2y")
    datum = (
        i,  # sales_id
        random.choice([visitor[0]
                      for visitor in fake_data['visitors']]),  # v_id
        math.ceil(
            datetime(date_sold.year, date_sold.month,
                     date_sold.day).timestamp()
        ),  # date_sold
        random.choice([staff[0] for staff in fake_data['staffs']]),  # v_id
    )
    fake_data['sales'].append(datum)

# console.log(fake_data['sales'][:100])
console.log("Done seeding sales!")
# ----------------------------------------------------

print("\n")

# seed sales_items
console.rule("[bold blue]Sales Items")

for i in track(range(NUM_STOCKS+1), description="seeding sales_items..."):
    stock_id = random.choice([stock[0] for stock in fake_data['stocks']])
    datum = (
        i,  # sales_id
        stock_id,  # stock_id
        random.randint(5, fake_data['stocks'][stock_id][4]-10),  # quantity
        fake_data['stocks'][stock_id][3],  # uprice
    )
    fake_data['sales_items'].append(datum)

# console.log(fake_data['sales_items'][:100])
console.log("Done seeding sales_items!")
# ----------------------------------------------------

print("\n")

# output seeds
console.rule("[bold red]Epilogue")
for k, v in fake_data.items():
    with console.status(f"seeding {str(k)}...", spinner="arc"):
        with open(f'db-{str(k)}.csv', 'x') as db:
            db.writelines(
                [','.join(
                    [str(datum) for datum in line]
                )+'\n' for line in v]
            )
# ----------------------------------------------------

console.print("[yellow]Done Seeding!")
