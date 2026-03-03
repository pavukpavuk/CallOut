# command to login as the postgres user. 
# -i (sim initial login) 
# -u (login as target user)
sudo -iu postgres

# command given to start the database
pg_ctl -D /var/lib/postgres/data -l logfile start

# edit file 
/var/lib/postgres/data/postgresql.conf
# to allow listening on network interfaces (os level)

# edit file 
/var/lib/postgres/data/pg_hba.conf 
# to authenticate certain connecting adresses (database level)


# list all databases 
psql -l 

# connect to database 
psql -d database_name  

# list db tables
\dt  <- lists tables 


# connect to database 
psql -d bookingsystem -U pavuk -h (database hostname) -p (database port)



