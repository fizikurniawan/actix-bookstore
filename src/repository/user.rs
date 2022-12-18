use crate::libs::constants;
use crate::models::pagination::SortingAndPaging;
use crate::models::response::Page;
use crate::models::user::{NewUser, UserDTO};
use crate::models::{filters::UserFilter, user::User};
use crate::schema::users::{self, dsl::*};
use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

impl User {
    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<User>, DbError> {
        let items = users.load::<User>(conn)?;
        Ok(items)
    }

    pub fn filter(filter: UserFilter, conn: &mut PgConnection) -> QueryResult<Page<User>> {
        let mut query = users::table.into_boxed();

        if let Some(i) = filter.email {
            query = query.filter(email.like(format!("%{}%", i)));
        }

        if let Some(i) = filter.name {
            query = query.filter(name.like(format!("%{}%", i)));
        }

        query
            .paginate(filter.page_num.unwrap_or(constants::DEFAULT_PAGE_NUM))
            .per_page(filter.page_size.unwrap_or(constants::DEFAULT_PER_PAGE))
            .sort(
                filter.sort_by.unwrap_or(constants::EMPTY_STR.to_string()),
                filter
                    .sort_direction
                    .unwrap_or(constants::EMPTY_STR.to_string()),
            )
            .load_and_count_items::<User>(conn)
    }

    pub fn find_by_id(i: i32, conn: &mut PgConnection) -> QueryResult<User> {
        users.find(i).get_result::<User>(conn)
    }

    pub fn insert(new_user: UserDTO, conn: &mut PgConnection) -> QueryResult<usize> {
        let target = NewUser {
            name: &new_user.name,
            email: &new_user.email,
            created_at: chrono::Utc::now().naive_utc(),
        };
        diesel::insert_into(users).values(&target).execute(conn)
    }

    pub fn update(i: i32, updated_user: UserDTO, conn: &mut PgConnection) -> QueryResult<usize> {
        diesel::update(users.find(i))
            .set(&updated_user)
            .execute(conn)
        // diesel::update(users::table)
        //             .filter(id.eq(id))
        //             .set(&updated_user)
        //             .execute(conn)
    }

    // pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
    //     diesel::delete(people.find(i)).execute(conn)
    // }
}
