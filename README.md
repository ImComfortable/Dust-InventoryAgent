Initial readme file, only for setup.


Agent features:

- Get some informations from computer: Hostname, User, ServiceTag(Dell only), Model, Version, IP, Disk Type, CPU, RAM, Monitor, ServiceTagMonitor(Dell Only) and last verfication time
- Power Off Computer if it's online for more three days

Features to think:

- Execute commands with cmd or powershell, i already made a prototype of this, but it's very unsafe and "amateur" to implementention
- Send warnings to us, about hardware and peripherals changes, for example keyboard swap or hdd changes

To do:

- [ ] Optimize the agent for every task
- [ ] Make a website for informations, to make my life more easy
- [ ] Create a login system for website with jws token
- [ ] Think about new info collections features for agent
- [ ] Test the agent in production, on a windows 10 and 11 Pcs


Server Things

- [ ] Configure the MongoDb, add an user and password for authentication
