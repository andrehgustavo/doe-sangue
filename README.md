# doe-sangue
Universidade Federal do Rio Grande do Norte 
Instituto Metrópole Digital 
Desenvolvimento de Sistemas Web II • DIM0547

## Project
Doe Sangue, Doe Vida

## Developer
 - Andre Gustavo C. M. de Barros

## Language
 - Rust

## Intro
In this programming project we will develop a system to facilitate blood donation. It will be possible for the user to follow the latest donations made, schedule donations, notify when the new donation is ready and monitor blood banks.

## User Requirements
 - As a user, I want to create my profile.
 - As a user, I want to login in my count.
 - As a user, I want to update my personal data.
 - As a user, I want to see my lastest donations.
 - As a user, I want to monitor blood banks. 
 - As a user, I want to schedulle a day to make my donation.
 - As a admin, I want to list all users.
 - As a admin, I want to delete some user.
 - As a admin, I want to supply the blood bank data.
 - As a admin, I want to provide the donation schedule.


## Endpoints
    - List all users ("/users")
    - Add new user ("/users/add")
    - Read a user ("/users/edit/id")
    - Update a user ("/users/edit/id")
    - Delete a user ("/users/delete/id")
    - Schedulle ("/users/id/schedulle)
    - Status ("/users/id/status")