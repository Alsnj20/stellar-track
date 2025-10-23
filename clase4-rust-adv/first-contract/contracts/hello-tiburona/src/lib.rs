// SETUP INITIAL
#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, contracttype, Env, Address, Symbol, String};

//DEFINE ERRORS
// A enum to represent possible errors in the contract
#[contracterror]
// Enum with specific error codes
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
// How to represent the enum in memory
#[repr(u32)]
pub enum HelloError {
    NameEmpty = 1,
    NameTooLong = 2,
    NotAuthorized = 3,
    NotInitialized = 4,
}

//DEFINE DATAKEY
// This enum to use as key or value in the contract storage
#[contracttype]
#[derive(Clone)]
// Customs keys for saving persistent data in the contract storage
pub enum DataKey {
    Admin,
    CountGreetings,
    CountGreetingsPerUser(Address),
    LastGreeting(Address),
}

//DEFINE CONTRACT
#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {

    pub fn initialize(env: Env, admin: Address) -> Result<(), HelloError> {
        //initialize contract
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(HelloError::NotInitialized);
        }
        
        //Save the admin
        env.storage()
            .instance()
        .set(&DataKey::Admin, &admin);

        //Initialize greeting count
        env.storage()
            .instance()
            .set(&DataKey::CountGreetings, &0u32);

        //Extend TTL
        env.storage()
            .instance()
            .extend_ttl(100, 100);

        Ok(())
    }

    // FASE 4

    pub fn hello(env: Env, user: Address, name: String) -> Result<Symbol, HelloError> {
        // Validate name
        if name.len() == 0 {
            return Err(HelloError::NameEmpty);
        }

        if name.len() > 32 {
            return Err(HelloError::NameTooLong);
        }

        // Increment greeting count
        let key_count = DataKey::CountGreetings;
        let count: u32 = env.storage()
            .instance()
            .get(&key_count)
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&key_count, &(count + 1));

        // RETO1: ADD INCREMENT COUNTER PER USER
        let key_count_user = DataKey::CountGreetingsPerUser(user.clone());
        let count_user: u32 = env.storage()
            .persistent()
            .get(&key_count_user)
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&key_count_user, &(count_user + 1));
        

        //Save last greeting of user
        env.storage()
            .persistent()
            .set(&DataKey::LastGreeting(user.clone()), &name);
        
        //Extend TTL
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::LastGreeting(user),100, 100);
        env.storage()
            .instance()
            .extend_ttl(100, 100);
        
        //Return greeting message
        Ok(Symbol::new(&env, "Hello"))
    }

    // FASE 5
    pub fn get_greeting_count(env: Env,) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::CountGreetings)
            .unwrap_or(0)
    }
    // RETO1: GET GREETING COUNT PER USER
    pub fn get_greeting_count_per_user(env: Env, user: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::CountGreetingsPerUser(user))
            .unwrap_or(0)
    }

    pub fn get_last_greeting(env: Env, user: Address) -> Option<String> {
        env.storage()
            .persistent()
            .get(&DataKey::LastGreeting(user))
    }


    // FASE 6
    pub fn reset_greeting_count(env: Env, caller: Address) -> 
    Result<(), HelloError> {
        // Check is caller is admin
        let admin: Address = env.storage().instance()
            .get(&DataKey::Admin)
            .ok_or(HelloError::NotInitialized)?;

        if caller != admin {
            return Err(HelloError::NotAuthorized)
        };

        // Reset greeting count 
        env.storage()
            .instance()
            .set(&DataKey::CountGreetings, &0u32);
        
        Ok(())
    }

    // RETO2: TRANSFER ADMIN
    pub fn transfer_admin(env:Env, caller:Address, new_admin:Address) -> 
    Result<(), HelloError> {
        // Check is caller is admin
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(HelloError::NotInitialized)?;

        if caller != admin {
            return Err(HelloError::NotAuthorized);
        }

        // Transfer admin
        env.storage()
            .instance()
            .set(&DataKey::Admin, &new_admin);

        Ok(())
    }
}   