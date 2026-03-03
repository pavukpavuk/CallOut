
CREATE TABLE IF NOT EXISTS useraccounts ( 
  account_id SERIAL PRIMARY KEY,      
  username VARCHAR(50) NOT NULL,    
  email VARCHAR(50) NOT NULL,
  pword VARCHAR(50) NOT NULL
);


SELECT * FROM UserAccounts;
