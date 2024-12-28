//!
//! Users are compactly represented by name, 
//! ID, and bitflags that represent their roles.
//! 
//! For compactness, we represent 128 roles, using u128
//! 

use core::hash;
use std::time::UNIX_EPOCH;
use nanoserde::{DeBin, SerBin};
use crate::{app_salt, blake3};


pub struct Password(String);

#[derive(DeBin, SerBin, Clone)]
pub struct User {
    legal_name: String,
    preferred_name: String,
    username: String,
    password_hash: [u8;32],
    roles: u128
}

#[derive(DeBin, SerBin, Clone)]
struct Roles {
    role_names: Vec<String>, // Need to maintain ordering.
    role_status: Vec<u8> // 0 --> disabled, 1 --> enabled
}

//
// We're using u8's because Rust may try to use u32 or u64 for bool... I see no reason to do that.
//
const ROLE_DISABLED: u8 = 0u8;
const ROLE_ENABLED: u8 = 1u8;






//
//
//
//
//        USERS STRUCT
//
//
//
//
#[derive(DeBin, SerBin, Clone)]
pub struct Users {
    users: Vec<User>,
    roles: Roles
}

impl Users {
    pub fn list_users(&self) -> Vec<User> {
        self.users.clone()
    }

    pub fn create_new_user(
        &mut self, 
        legal_name: String, 
        preferred_name: String, 
        username: String,
        roles: u128
    ) -> Result<Password, UserError> {
        //
        // Check we don't have overlap with other username.
        // This isn't most efficient for lookups, but should be okay up to
        // a few thousand users. We're trading space for compute.
        // Beyond that, switch to keeping a standalone hashset for 
        // constant lookup.
        //
        let overlaps : Vec<&User> = self.users.iter().filter(|user|{
            user.username == username
        }).collect();

        if overlaps.len() > 0 {
            return Err(UserError::UsernameAlreadyExists); // TODO: use error enum
        }

        let new_password : [u8;32] = {
            let mut hasher = blake3::Hasher::new();
            let current_time = std::time::SystemTime::now();
            let current_time = current_time.duration_since(UNIX_EPOCH).unwrap().as_micros();
            
            hasher.update(&current_time.to_be_bytes());
            hasher.update(&app_salt());
            hasher.update(username.as_bytes());

            let mut pass : [u8;32] = [0u8;32];
            hasher.finalize(pass.as_mut_slice());
            pass
        };
        let new_password = String::from_utf8(new_password.to_vec()).unwrap();

        let password_hash = {
            let mut hasher = blake3::Hasher::new();
            hasher.update(new_password.as_bytes());
            hasher.update(&app_salt());

            let mut out_slice: [u8;32] = [0u8;32];
            hasher.finalize(out_slice.as_mut_slice());
            out_slice
        } ;


        // Build user object
        let new_user = User {
            legal_name,
            preferred_name,
            username,
            roles,
            password_hash
        };

        // Store it. Write on update.
        self.users.push(new_user);
        match std::fs::write("userstore.bin", self.users.serialize_bin()) {
            Ok(_) => return Ok( Password(new_password) ),
            Err(_e) => return Err(UserError::FailedWriteToDisk),
        }
    }

    pub fn list_all_roles(&self) -> Roles {
        self.roles.clone()
    }

    pub fn create_new_role(&mut self, role_name: String) -> Result<(), UserError> {
        // For sake of this tiny app, we're limited to 128 roles or fewer.
        if self.roles.role_names.len() >= 128 {
            return Err(UserError::RoleLimitReached);
        }
        
        // Check if we have this role already
        let lower_form = role_name.to_lowercase();
        let mut exists = false;

        for role in self.roles.role_names.iter() {
            if role.to_lowercase() == lower_form {
                exists = true;
                break;
            }
        }

        if exists {
            return Err(UserError::RoleAlreadyExists)
        }

        //
        // Create new role, making disabled by default
        //
        self.roles.role_names.push(role_name);
        self.roles.role_status.push(ROLE_DISABLED);

        Ok(())
    }

    pub fn apply_role_to_user(&mut self, role_index: usize, user_index: usize) -> Result<(), UserError> {
        if role_index > 128 || role_index > self.roles.role_names.len() {
            return Err(UserError::InvalidRole)
        }

        // Check if user already has the role applied
        let targeted_bit : u128 = 1 << role_index;
        if (targeted_bit & self.users[user_index].roles) > 0 {
            return Err(UserError::RoleAlreadyApplied);
        } else {
            // Bitwise OR it.
            self.users[user_index].roles = self.users[user_index].roles | targeted_bit;
            Ok(())
        }
    }

    pub fn remove_role_from_user(&mut self, role_index: usize, user_index: usize) -> Result<(), UserError> {
        if role_index > 128 || role_index > self.roles.role_names.len() {
            return Err(UserError::InvalidRole)
        }

        // Check if user already has the role applied
        let targeted_bit : u128 = 1 << role_index;
        let user_roles = self.users[user_index].roles;
        if (targeted_bit & user_roles) > 0 {
            // Bitwise XOR it.
            self.users[user_index].roles = user_roles ^ targeted_bit;
            Ok(())
        } else {
            return Err(UserError::RoleNotPresent);
        }
    }

    pub fn check_role_against_user(&self, role_index: usize, user_index: usize) -> bool {
        (
            self.users[user_index].roles & ( 1 << role_index )
        ) > 0
    }
}

#[derive(Debug)]
pub enum UserError {
    UsernameAlreadyExists,
    FailedWriteToDisk,
    RoleAlreadyExists,
    RoleLimitReached,
    RoleAlreadyApplied,
    InvalidRole,
    RoleNotPresent,
}