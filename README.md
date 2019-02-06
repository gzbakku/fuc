# Fuc* DB

[![N|Solid](https://www.rust-lang.org/static/images/rust-logo-blk.svg)](https://nodesource.com/products/nsolid)

[![Build Status](https://travis-ci.org/joemccann/dillinger.svg?branch=master)](https://travis-ci.org/joemccann/dillinger)

Fuc* Db is an open source NoSQL json object Database written in rust.  

  - Native horizontal and vertical scaling.
  - Fast Pre Compiled Query System
  - Super fast read and write
  - Open source alternative to Firebase Firestore

# New Features!

  - Multi doc insert and delete functions.
  - Queries can now be fetched in sections by limiting docs.
  - Delete Collection from a single request.

### Installation

download the built directory and run the windows executable and you have your db.

### Usage

Fuc Db use a simple rest api to interface which can be queried with http post request.

## Init

# Initiate a db 
put the windows executable in the folder where you want the db to be built and post the following json to initiate your db.

Url : http://localhost:3000/init

```sh
{
    base:'rabdombasestring'
}
```

#### Response

```sh 
{
    result:true,
    error:null,
    docs:{
        key:'sd897sdf897sd89f7sd98f7sd98f7sd89f789sdf'
    },
    message:null
}
```

this base key value pair should be a random string and will be used to produce a admin key to the db. 

# User Account Queries
the following queries can be used to manage user accounts.

## Register a user 
the user account can be granted priveledges to submit and remove the files from the database.

Url : http://localhost:3000/user/register

```sh
{
    key:'generated key',
    user:'userNameString',
    password:'userPasswordString'
}
```

#### Response

```sh 
{
    result:true,
    error:null,
    docs:null,
    message:null
}
```

## Reset User Account
the user account can be reset by the following request

Url : http://localhost:3000/user/reset

```sh
{
    key:'generated key',
    user:'registeredUserNameString',
    password:'newUserPasswordString'
}
```

#### Response

```sh 
{
    result:true,
    error:null,
    docs:null,
    message:null
}
```

## Delete User Account
Deleting the user account if you want to.

Url : http://localhost:3000/user/delete

```sh
{
    key:'generated key',
    user:'registeredUserNameString'
}
```

#### Response

```sh 
{
    result:true,
    error:null,
    docs:null,
    message:null
}
```

## Start a Session
session is required to start transaction in the database,session dont have a time limit and can be ended when a new session is started. session is recognised by a token limited to a single ip.This token have to be stored locally to transact with the db.

Url : http://localhost:3000/user/connect

```sh
{
    user:'registeredUserNameString',
    password:'userPasswordString'
}
```

#### Response

```sh 
{
    result:true,
    error:null,
    docs:{
        user:'akku',
        token:'89a7s89sa7d89as7d89as7das89d7'
    },
    message:null
}
```

# Insert Multiple Json Document
Multiple Json objets can be inserted, 100 at a time. wait for the previous query to complete before making another query.

Url : http://localhost:3000/insert

```sh
{
    user:'usernameString',
    token:'generatedToken',
    address:'collectionAddressString',
    docs:[{
        something:'special'
    }]
}
```

#### Response

```sh 
{
    result:true,
    error:null,
    docs:null,
    message:null
}
```

# Query 

## Register A Index
before running a query an index is needed to be made. Fuc use a precompiled data structure to provide fast and scalable query system.

#### Query Structure
query includes 3 components equal,weight,order and search. with data type attached.

#### Equal function
### name.string.equal
this string cantains the key of the key value pair in json object as the first argument, followed by data type and seprated by a "." and the function

#### Weight Function
### pound.number.weight
this function make a index to do >= <= functions with numbers, the first argument is the key followed by the data type and the query function. this function can only be called on keys with data type of a number.

#### Search Function
### name.string.search
this function is similar to the above two.

#### Order Function
### regDate.asc
this function is called by the key followed by the direction of which the data is listed in the database. valid directions are asc and desc.This function is seprated by "_" in the query string.

#### Query with Multiple Functions

### name.string.equal||mobile.number.equal_regDate.asc

weight,search and equal function are sepereated by "||" and the order function is to be placed at the end of the query seperated by "_".

#### Query Validity
multiple functions can be combined to make a desired index but only one of dynamic function is allowed in the query. this means that either one of search,order or weight can be used with multiple equal tags.

#### Valid
### name.string.equal||mobile.number.equal_regDate.asc

#### Invalid
### name.string.equal||mobile.number.equal||city.string.search_regDate.asc

Url : http://localhost:3000/query/register

```sh
{
    user:'akku',
    token:'ds89f7sd89f7sd89f7sd89f7s89df',
    query:'name.string.equal||mobile.number.equal_regDate.asc',
    address:'collectionAddress'
}
```

#### Response

```sh 
{
    result:true,
    error:null,
    docs:null,
    message:null
}
```

## Query Run
queries can be used to find data in the db with minimum cost by the following request.

```sh
{
    user:'akku',
    token:'ds89f7sd89f7sd89f7sd89f7s89df',
    query:'name.string.equal||mobile.number.equal_regDate.asc',
    address:'collectionAddress',
    params:{
        name:'akku',
        mobile:91100110011,
        regDate:5645678453468464,
        dir:'asc',
        limit:25,
        last:'dsa7fds7f6sd7f868'
    }
}
```

#### Response

```sh 
{
    result:true,
    error:null,
    docs:['098sdf9sd8f88dfd','sd09f8sd90f8ds908f'],
    message:null
}
```

this api returns the doc address which can be fetched afterwards.

params document containes the key value pair of the search tag and value with direction (dir) => asc or desc, limit is the number of documents you want to query and last is the last name value from the doc from previous query.

## Get Queries

### Get Docs By Id
this query returns a array of json objects by document Id.manimum of 50 docs can be fetched at once. multiple fetch queries can be called at once depends on your storage read speed.

url : http://localhost:3000/get/docs

```sh 
{
    user:'akku',
    token:'ds89f7sd89f7sd89f7sd89f7s89df',
    address:'collectionAddress',
    docs:['sd789sdf8sd8f','sdfsdfsdf890']
}
```

#### Response

```sh 
{
    result:true,
    error:null,
    docs:[{doc:1},{doc:2}],
    message:null
}
```

### Get Docs In a Collection
this query returns doc id of all the docs in a collection which can be listed by using control params of direction(dir), limit and last.multiple queries can be called at once.if you wanna get all the docs just dont use the params.

```sh 
{
    user:'akku',
    token:'ds89f7sd89f7sd89f7sd89f7s89df',
    address:'collectionAddress'
}
```

#### Response

```sh 
{
    result:true,
    error:null,
    docs:['sd789sdf8sd8f','sdfsdfsdf890'],
    message:null
}
```

these doc ids can be canverted to docs using get docs query.

## Delete

### Delete Docs
this query takes a array of doc ids and deletes em all at max 100 docs can be deleted at once dont make more then 50 simutanious request at once well this depends on your io speeds.

```sh 
{
    user:'akku',
    token:'ds89f7sd89f7sd89f7sd89f7s89df',
    address:'collectionAddress',
    docs:['sd789sdf8sd8f','sdfsdfsdf890']
}
```

#### Response

```sh 
{
    result:true,
    error:null,
    docs:null
    message:null
}
```

### Delete Collection
this query takes a collection address and deletes all the data inside of it including child components for this query you have to provide admin privileges to 
the app on your os.At max 10 simultanious queries can be made.

```sh 
{
    user:'akku',
    token:'ds89f7sd89f7sd89f7sd89f7s89df',
    address:'collectionAddress',
}
```

#### Response

```sh 
{
    result:true,
    error:null,
    docs:null
    message:null
}
```

License
----

MIT

**You be free Hackers **

fuc* by Akku
