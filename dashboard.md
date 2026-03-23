# Ducks in a row dashboard.

A web app using Rocket.rs 


9 widgets in a 3 by 3 grid

| 1 | 2 | 3 |
| - | - | - |
| 4 | 5 | 6 |
| - | - | - |
| 7 | 8 | 9 |


Data for each is loaded with jQuery calls to backend.

1. wazuh
2. Atera
3. Vipre AV
4. Backup
5. AD metrics
6. N-central
7. Veeam jobs
8. Weather info
9. Duck showing date and time


## Wazuh
- critical alerts
- high alerts

And a stacked bar graph for the highest value for each over the last 7 days per day

Updates hourly


## Atera
- active alerts
- open tickets
- patching status (pending patches / device count)

And a line graph for the highest value for alerts and ticket count over the last 7 days per day

Updated every 10 minutes


## Vipre AV
- outdated devices count
- lost devices count
- devices in AD, but no AV
- devices with AV not in AD

And a line graph for the highest value for each over the last 7 days per day


## Backup
A pie graph of devices with status, green, yellow, orange, red and grey. Highlight/pop out those not green.

Updates bi-hourly (every 2 hours)


## AD Metrics
- number of accounts with password expired
- number of locked out accounts

And a line graph for the highest value for each over the last 7 days per day

Updates bi-hourly (every 2 hours)


## N-central
listing of all alerts from N-central
device name, service, transition time, type

Updates every 5 minutes


## Veeam jobs
A pie graph of devices with status, green, yellow, orange, red and grey. Highlight/pop out those not green.

Updates bi-hourly (every 2 hours)


## Weather info
A graphical weather widget with temperature


## Duck showing date and time
Duck with an emoji like date sign and current time below


