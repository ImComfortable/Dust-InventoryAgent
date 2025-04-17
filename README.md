# Main purpose
Build a simple agent who can be integraded with a platform, and work together to automate inventory management and give us a help in audict.

I already make a "plataform", is simple, but usefull: [Link](https://github.com/ImComfortable/Site-agent/tree/main)

# Agent features:
- PC Model
- Hostname
- Local IP
- Monitor Model
- Service Tag (PC and Monitor)
- Windows version
- If windows is active
- CPU
- Drive Type and Size
- RAM
- Restart Computer if it's online for more three days
- Get all installed programs from computer
- Get all acessed pages on browser and time you spent on it

# Features to think:

- Add servicetag on page capture
- System to define horary for set departament work time, it will help to define a hour to shutdown the computers
- Configuration page
- Filter and some window in actives page to separeted inactive computers from active computers
- Comment all code, to make it more understandable 
- Remove App list from audict page and add it on actives page
- Make a separed area to admin and gestors, gestors can login to view some reports and insights
- Build msi to install agent for compunter betwen user

# MSI Template
This file is a simple template to build a MSI file, you only need wix installed on your computer, just compile and build.
[Template used to make a base](https://github.com/letsdoautomation/wix-toolset-4-cli/tree/49292c799901d8dc69b68e20bdf204c454637a8f/Create%20Visual%20Studio%20Code%20MSI%20installation%20file)  
