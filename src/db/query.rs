#![allow(unused_imports)]
use crate::schema::speakers;
use crate::sms::sms::{SmsClient, SpeakerResponse};
use actix::fut::ok;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use clap::builder::Str;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Connection, ExpressionMethods, Insertable, PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use dotenv::dotenv;
use log::debug;
use std::env;
use std::error::Error;
use std::fmt::format;
use std::time::Duration;

#[derive(Debug)]
pub struct DatabasePool{
    pool:Pool<ConnectionManager<PgConnection>>
}


pub struct SpeakerManger{
    db_pool:DatabasePool,
    sms_client:SmsClient,
}

impl SpeakerManger{
    pub fn new(db_pool: DatabasePool,sms_client: SmsClient)->Self{
        SpeakerManger{db_pool,sms_client}
    }

    //add a new speaker, saves to the database, and sends a sms notification.
    pub async fn add_speaker<'a>(&self,speaker: Speaker)->Result<(), Box<dyn Error>>{
        //save to the database
        self.db_pool.add_speaker((&speaker.name).parse().unwrap(), (&speaker.phone_number).parse().unwrap(), speaker.assigment_date);

        //Send Sms notification
        let body=format!("Hello {}, You have been added as a speaker. Your assigment is {}.",speaker.phone_number,speaker.assigment_date);
        self.sms_client.send_sms(&speaker.phone_number,&body).await?;

        debug!("Speaker added and notification sent");

        Ok(())
    }

    pub async fn process_response(&self,response:SpeakerResponse)->Result< (), Box<dyn Error>>{
    match response{
        SpeakerResponse::Accepted(phone_number)=>{
            debug!("speaker at {}, accepted the request",phone_number);
            self.db_pool.mark_speaker_as_notified(&phone_number)?;
        }

        SpeakerResponse::Denied(phone_number)=>{
            debug!("Speaker at {} denied the request",phone_number);
            self.db_pool.delete_speaker(&phone_number)?;
        }

    }
        Ok(())
    }
}

impl DatabasePool{

    pub fn add_speaker(&self, name: String, phone_number:String, assigment_date: NaiveDateTime) -> Speaker {

        let mut conn =self.get_connection_pool().expect("Failed to get a connection from pool");
        let new_speaker= NewSpeaker{
            name,
            phone_number,
            assigment_date,
            notified: false,
        };

        diesel::insert_into(speakers::table).values(&new_speaker).returning(Speaker::as_returning()).get_result(&mut conn).expect("Error saving a new speaker")

    }
    pub fn new() ->Result<Self,Box<dyn  Error> >{


        //Load environment variables
   dotenv().ok();

        //Read the URl from the environment
        let database_url=env::var("PG_DATABASE_URL").or_else(|_| env::var("DATABASE_URL")).expect("DATABASE_URL must be set");



        //Create a connection manager and build the pool
        let manager=ConnectionManager::<PgConnection>::new(database_url);
        let pool=Pool::builder().max_size(15).min_idle(Some(5)).idle_timeout(Some(Duration::from_secs(300))).build(manager)?;


        Ok(DatabasePool{pool})
    }


    pub fn get_connection_pool(&self) -> Result<diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>>, Box<dyn Error>> {
       self.pool.get().map_err(|e|e.into())

    }

    pub fn save_speakers(conn: &mut PgConnection,name: String, phone_number: String,assigment_date: NaiveDateTime, notified:bool)->Speaker{
        use crate::schema::speakers;

        let new_speaker= NewSpeaker{
            name,
            phone_number,
            assigment_date,
            notified
        };

        diesel::insert_into(speakers::table).values(&new_speaker).returning(Speaker::as_returning()).get_result(conn).expect("Error saving a new speaker")
    }

    pub fn mark_speaker_as_notified(&self, phone_number: &str)->Result<(), Box<dyn  Error>>{
        let mut conn =self.get_connection_pool()?;
        diesel::update(speakers::table.filter(speakers::phone_number.eq(phone_number))).set(speakers::notified.eq(true)).execute(&mut conn)?;

        debug!("Speaker with phone number {} marked as notified", phone_number);

        Ok(())
    }

    pub fn delete_speaker(&self, phone_number: &str)->Result<(),Box<dyn Error>>{
        let mut conn =self.get_connection_pool()?;
        diesel::delete(speakers::table.filter(speakers::phone_number.eq(phone_number))).execute(&mut conn)?;

        debug!("Speaker with phone number {} removed from the database",phone_number);

        Ok(())

    }

}





#[derive(Debug, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = speakers)]
pub struct Speaker {
    pub id: i32,
    pub name: String,
    pub phone_number:String,
    pub assigment_date: NaiveDateTime,


}

#[derive(Insertable)]
#[diesel(table_name=speakers)]
pub struct NewSpeaker{
    pub name: String,
    pub phone_number: String,
    pub assigment_date: NaiveDateTime,
    pub notified: bool
}

