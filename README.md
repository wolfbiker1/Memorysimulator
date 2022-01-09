# About
This application arose from the idea of ​​developing a memory management system (dynamic malloc and free operations) for an embedded device.
In order to prevent to many erase and write cycles for the flash memory I wrote this tool, which simulates the sram memory of
the device in the best possible way. The project is still in the early stages, so the system only works with 4 byte aligned addresses so far.
It offers basic write- and readoperations.