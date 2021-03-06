// Copyright 2018-2020 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::commits::store::diesel::schema::{chain_record::dsl::*, commits::dsl::*};
#[cfg(feature = "location")]
use crate::locations::store::diesel::schema::{location::dsl::*, location_attribute::dsl::*};
#[cfg(all(feature = "pike", feature = "location"))]
use crate::pike::store::diesel::schema::pike_organization_location_assoc::dsl::*;
#[cfg(feature = "pike")]
use crate::pike::store::diesel::schema::{
    pike_agent::dsl::*, pike_agent_role_assoc::dsl::*, pike_allowed_orgs::dsl::*,
    pike_inherit_from::dsl::*, pike_organization::dsl::*, pike_organization_alternate_id::dsl::*,
    pike_organization_metadata::dsl::*, pike_permissions::dsl::*, pike_role::dsl::*,
};
#[cfg(feature = "product")]
use crate::products::store::diesel::schema::{product::dsl::*, product_property_value::dsl::*};
#[cfg(feature = "schema")]
use crate::schemas::store::diesel::schema::{
    grid_property_definition::dsl::grid_property_definition, grid_schema::dsl::*,
};
#[cfg(feature = "track-and-trace")]
use crate::track_and_trace::store::diesel::schema::{
    associated_agent::dsl::*, property::dsl::*, proposal::dsl::*, record::dsl::*,
    reported_value::dsl::*, reporter::dsl::*,
};

use diesel::RunQueryDsl;
#[cfg(feature = "sqlite")]
use diesel::{sqlite::SqliteConnection, Connection};

use crate::error::ResourceTemporarilyUnavailableError;
use crate::migrations::error::MigrationsError;

embed_migrations!("./src/migrations/diesel/sqlite/migrations");

/// Run database migrations to create Grid tables
///
/// # Arguments
///
/// * `conn` - Connection to database
///
#[cfg(all(feature = "sqlite", feature = "diesel"))]
pub fn run_migrations(conn: &SqliteConnection) -> Result<(), MigrationsError> {
    embedded_migrations::run(conn).map_err(|err| {
        MigrationsError::ResourceTemporarilyUnavailableError(
            ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
        )
    })?;

    info!("Successfully applied Grid migrations");

    Ok(())
}

#[cfg(all(feature = "sqlite", feature = "diesel"))]
pub fn clear_database(conn: &SqliteConnection) -> Result<(), MigrationsError> {
    conn.transaction::<_, MigrationsError, _>(|| {
        #[cfg(feature = "pike")]
        {
            diesel::delete(pike_agent).execute(conn)?;
            diesel::delete(pike_inherit_from).execute(conn)?;
            diesel::delete(pike_permissions).execute(conn)?;
            diesel::delete(pike_allowed_orgs).execute(conn)?;
            diesel::delete(pike_agent_role_assoc).execute(conn)?;
            diesel::delete(pike_organization_metadata).execute(conn)?;
            diesel::delete(pike_organization_alternate_id).execute(conn)?;
            diesel::delete(pike_organization).execute(conn)?;
            diesel::delete(pike_role).execute(conn)?;
        }
        #[cfg(feature = "location")]
        {
            diesel::delete(location_attribute).execute(conn)?;
            diesel::delete(location).execute(conn)?;
        }
        #[cfg(all(feature = "pike", feature = "location"))]
        {
            diesel::delete(pike_organization_location_assoc).execute(conn)?;
        }
        #[cfg(feature = "product")]
        {
            diesel::delete(product).execute(conn)?;
            diesel::delete(product_property_value).execute(conn)?;
        }
        #[cfg(feature = "schema")]
        {
            diesel::delete(grid_property_definition).execute(conn)?;
            diesel::delete(grid_schema).execute(conn)?;
        }
        #[cfg(feature = "track-and-trace")]
        {
            diesel::delete(associated_agent).execute(conn)?;
            diesel::delete(property).execute(conn)?;
            diesel::delete(proposal).execute(conn)?;
            diesel::delete(record).execute(conn)?;
            diesel::delete(reported_value).execute(conn)?;
            diesel::delete(reporter).execute(conn)?;
        }
        diesel::delete(chain_record).execute(conn)?;
        diesel::delete(commits).execute(conn)?;

        Ok(())
    })?;

    Ok(())
}
