extern crate compose_errors;

use compose_errors::{compose_errors_fn};

fn main() {
    let result = frobnicate();

    match result {
        Ok(_) => println!("Success"),
        Err(e) => match e {
            FrobnicationError::FrobnicationError__persistence_PersistenceError(e) =>
                println!("Persistence error! {:?}", e),

            FrobnicationError::FrobnicationError__service_a_client_ServiceAError(e) =>
                println!("ServiceAClient error! {:?}", e),

            FrobnicationError::FrobnicationError__service_b_client_ServiceBError(e) =>
                println!("ServiceBClient error! {:?}", e),
        }
    }
}

#[compose_errors_fn(
    persistence::PersistenceError,
    service_a_client::ServiceAError,
    service_b_client::ServiceBError
)]
#[derive(Debug)]
enum FrobnicationError{}

fn frobnicate() -> Result<i32, FrobnicationError> {
    let some_db_value = persistence::read_from_database()?;
    let some_service_a_value = service_a_client::read_something()?;
    let some_service_b_value = service_b_client::write_something(some_db_value, some_service_a_value)?;
    let result = persistence::write_to_database(some_service_b_value)?;

    Ok(result)
}

mod persistence {
    use std::io;

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum PersistenceError {
        IOError(io::Error),
        OptimisticConcurrencyFailure
    }

    pub fn read_from_database() -> Result<i32, PersistenceError> {
        //do something here that possibly returns one of the above errors
        Ok(1)
    }
    pub fn write_to_database(_: i32) -> Result<i32, PersistenceError> {
        //do something here that possibly returns one of the above errors
        Ok(1)
    }
}

mod service_a_client {
    use std::io;

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum ServiceAError {
        IOError(io::Error),
        ServiceADomainError1,
        ServiceADomainError2
    }

    pub fn read_something() -> Result<i32, ServiceAError> {
        //do something here that possibly returns one of the above errors
        Ok(2)
    }
}

mod service_b_client {
    use std::io;

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum ServiceBError {
        IOError(io::Error),
        ServiceBDomainError1,
        ServiceBDomainError2
    }

    pub fn write_something(_: i32, _: i32) -> Result<i32, ServiceBError> {
        //do something here that possibly returns one of the above errors
        Ok(3)
    }
}
