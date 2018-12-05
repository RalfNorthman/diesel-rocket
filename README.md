## Diesel-Rocket - Http CRUD

You will need a 'Rocket.toml' file so Rocket can find your MariaDB.
It should look something like this:

```
[global.databases]
my_db = { url = "mysql://user:password@host/database"}
```

`my_db` is a name used in the Rust code. 
If you change it in your Rocket.toml, 
you have to change it in the code too.

You will also need a '.env' file for the diesel command line tool.

It should look something like this:

```
DATABASE_URL=mysql://user:password@host/database
```
