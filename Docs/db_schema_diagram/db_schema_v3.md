// Use DBML to define your database structure
// Docs: https://dbml.dbdiagram.io/docs


//if gym is deleted, fighters gym is nulled
Table gyms {
  id integer [primary key]
  name varchar
  created_at timestamp
  gym_logo_url varchar

}

//if fighter is deleted, all callouts and events are nulled
Table fighters {
  id integer [primary key]
  name varchar
  rank varchar
  created_at timestamp
  wins int
  losses int
  draws int
  weight int
  profile_picture_url varchar
  gym_id_fk int
  user_id_fk int unique
}

Table useraccounts{
  id integer [primary key]
  username varchar unique
  password_hash varchar
  verified boolean
  email_nonce bytes
  user_key bytes
  user_key_nonce bytes
  email bytes unique
}

Table sessions{
  id integer [primary key]
  session_id bytes unique
  user_id_fk integer //cascade on delete
  created_at timestamp
  expires_at timestamp
  last_seen_at timestamp
}

//not yet
// Table FighterCallouts {
//   fighter_id_fk int
//   callout_id_fk int
//   role enum // caller, responder, participant

//   Indexes {
//     (fighter_id_fk, callout_id_fk) [pk]
//   }
// }

//if callouts are deleted events null callouts
Table callouts {
  id integer [primary key]
  title varchar
  location varchar
  status enum //pending, rejected, accepted
  date datetime
  created_at timestamp
  //last two cannot be the same
  caller_id_fk int
  responder_id_fk int
}

Table event{
  id integer [primary key]
  title varchar
  date_of_event datetime
  location varchar
  win_type enum //submission, points, DQ, etc
  winner_id_fk int
  loser_id_fk int
  callout_id_fk int 
}

Ref: fighters.gym_id_fk > gyms.id
Ref: fighters.user_id_fk - useraccounts.id

Ref: sessions.user_id_fk > useraccounts.id

// Ref: FighterCallouts.fighter_id_fk - Fighters.id
// Ref: FighterCallouts.callout_id_fk - Callouts.id
Ref: callouts.caller_id_fk - fighters.id
Ref: callouts.responder_id_fk - fighters.id

Ref: event.winner_id_fk - fighters.id
Ref: event.loser_id_fk - fighters.id
Ref: event.callout_id_fk < callouts.id