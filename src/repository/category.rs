use crate::libs::constants;
use crate::models::category::{Category, CategoryDTO, NewCategory};
use crate::models::filters::CategoryFilter;
use crate::models::pagination::{SortingAndPaging};
use crate::models::response::Page;
use crate::schema::categories::{self, dsl::*};

use diesel::{PgConnection, QueryDsl};
use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

impl Category {
    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<Category>, DbError> {
        let items = categories.load::<Category>(conn)?;
        Ok(items)
    }

    pub fn filter(filter: CategoryFilter, conn: &mut PgConnection) -> QueryResult<Page<Category>> {
        let mut query = categories::table.into_boxed();

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
            .load_and_count_items::<Category>(conn)
    }

    pub fn find_by_id(i: i32, conn: &mut PgConnection) -> QueryResult<Category> {
        categories.find(i).get_result::<Category>(conn)
    }

    pub fn insert(new_category: CategoryDTO, conn: &mut PgConnection) -> QueryResult<usize> {
        let target = NewCategory {
            name: &new_category.name,
            created_at: chrono::Utc::now().naive_utc()
        };
        diesel::insert_into(categories).values(&target).execute(conn)
    }

    pub fn delete(i: i32, conn: &mut PgConnection) -> QueryResult<usize> {
        diesel::delete(categories.find(i)).execute(conn)
    }

    pub fn update(i: i32, updated_category: CategoryDTO, conn: &mut PgConnection) -> QueryResult<usize> {
        diesel::update(categories.find(i))
            .set(&updated_category)
            .execute(conn)
    }
    
}
