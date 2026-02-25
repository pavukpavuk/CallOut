// Use DBML to define your database structure
// Docs: https://dbml.dbdiagram.io/docs


//if gym is deleted, fighters gym is nulled
Table Gyms {
  id integer [primary key]
  name varchar
  created_at timestamp

}

//if fighter is deleted, all callouts and events are nulled
Table Fighters {
  id integer [primary key]
  name varchar
  rank varchar
  created_at timestamp
  wins int
  losses int
  draws int
  gym_id_fk int

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
Table Callouts {
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

Table Event{
  id integer [primary key]
  title varchar
  date_of_event datetime
  location varchar
  win_type enum //submission, points, DQ, etc
  winner_id_fk int
  loser_id_fk int
  callout_id_fk int 
}

Ref: Fighters.gym_id_fk > Gyms.id

// Ref: FighterCallouts.fighter_id_fk - Fighters.id
// Ref: FighterCallouts.callout_id_fk - Callouts.id
Ref: Callouts.caller_id_fk - Fighters.id
Ref: Callouts.responder_id_fk - Fighters.id

Ref: Event.winner_id_fk - Fighters.id
Ref: Event.loser_id_fk - Fighters.id
Ref: Event.callout_id_fk < Callouts.id