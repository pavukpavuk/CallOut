===== 26-09-25 =====
created a small user table in database. subject to change. 
Loading db link in from .env

===== 09-10-25 =====
set up raspberry pi database server in house
allowing connections from my network subnet.
set up ssh also 


# Email adresses are encrypted and stored in the user struct encoded as hexidecimal ciphertext. T
# the nonce is stored alongside it. 
# The key for each user is stored encrypted alongside the nonce in the user struct also.
# Each user key is encrypted using the master key. 