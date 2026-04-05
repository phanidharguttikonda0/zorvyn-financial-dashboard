use std::time::Duration;
use sqlx::pool::PoolOptions;
use sqlx::{Pool, Postgres, Row};
use sqlx::postgres::PgPoolOptions;
use crate::models::authentication_models::{UserInfo};
use crate::models::category_models::Category;
use crate::models::counterparty_models::Party;
use crate::models::user_models::User;
use tracing::{info, warn};
use crate::services::authentication_services::hash_password;

#[derive(Debug, Clone)]
pub struct DBService {
    pub connection: Pool<Postgres>,
}


impl DBService {
    pub async fn new() -> DBService {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
        .max_connections(10)
            .acquire_timeout(Duration::from_secs(5))
        .connect(&url)
        .await
        .expect("Failed to connect to the database") ;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run database migrations");

        DBService {
            connection: pool
        }
    }

    pub async fn create_user(&self, user: User) -> Result<(), sqlx::Error> {
        let role = user.role.unwrap_or_else(|| "viewer".to_string());
        let status = user.status.unwrap_or_else(|| "active".to_string());
        let name = user.name.unwrap_or_default();
        let email = user.email.unwrap_or_default();
        // Assuming the password must be provided, otherwise we store an empty string or error.
        // Usually creation should be validated in controller, but just to be safe.
        let password = user.password.unwrap_or_else(|| "password".to_string());
        let hashed_password = hash_password(&password);

        sqlx::query(
            "INSERT INTO users (name, email, password_hash, role, status) VALUES ($1, $2, $3, $4::user_role, $5::user_status)"
        )
            .bind(name)
            .bind(email)
            .bind(hashed_password)
            .bind(role)
            .bind(status)
            .execute(&self.connection)
            .await?;

        Ok(())
    }

    pub async fn update_user(&self, id: i64, user: User) -> Result<(), sqlx::Error> {
        let hashed_password = user.password.map(|p| hash_password(&p));
        sqlx::query(
            "UPDATE users \
             SET name = COALESCE($1, name), \
                 email = COALESCE($2, email), \
                 password_hash = COALESCE($3, password_hash), \
                 role = COALESCE($4::text::user_role, role), \
                 status = COALESCE($5::text::user_status, status) \
             WHERE id = $6"
        )
            .bind(user.name)
            .bind(user.email)
            .bind(hashed_password)
            .bind(user.role)
            .bind(user.status)
            .bind(id)
            .execute(&self.connection)
            .await?;

        Ok(())
    }

    pub async fn get_all_users(&self, last_id: Option<i64>, limit: i64) -> Result<Vec<User>, sqlx::Error> {
        let last_id = last_id.unwrap_or(0);
        
        let records = sqlx::query(
            "SELECT id, name, email, role::TEXT, status::TEXT FROM users WHERE id > $1 ORDER BY id ASC LIMIT $2"
        )
            .bind(last_id)
            .bind(limit)
            .fetch_all(&self.connection)
            .await?;

        let users = records.into_iter().map(|row| {
            let id: i32 = row.get("id");
            User {
                id: Some(id as i64),
                name: Some(row.get("name")),
                email: Some(row.get("email")),
                password: None,
                role: Some(row.get("role")),
                status: Some(row.get("status")),
            }
        }).collect();

        Ok(users)
    }

    pub async fn check_admin_and_init(&self) {
        let admin_exists = sqlx::query("SELECT 1 FROM users WHERE role = 'admin'")
            .fetch_optional(&self.connection)
            .await
            .unwrap_or(None)
            .is_some();

        if admin_exists {
            info!("Admin user already exists. Skipping initialization.");
            return;
        }

        info!("No admin user found. Attempting to create one from .env.");
        let admin_name = std::env::var("ADMIN_NAME").unwrap_or_else(|_| "Admin".to_string());
        let admin_email = std::env::var("ADMIN_EMAIL").unwrap_or_else(|_| "admin@example.com".to_string());
        let admin_password = std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "password".to_string());
        
        let hashed_password = hash_password(&admin_password);

        let result = sqlx::query(
            "INSERT INTO users (name, email, password_hash, role, status) VALUES ($1, $2, $3, 'admin'::user_role, 'active'::user_status) ON CONFLICT (email) DO NOTHING"
        )
            .bind(&admin_name)
            .bind(&admin_email)
            .bind(&hashed_password)
            .execute(&self.connection)
            .await;

        match result {
            Ok(_) => info!("Admin user created successfully from .env details."),
            Err(e) => warn!("Failed to create admin user: {:?}", e),
        }
    }

    pub async fn get_user_password(&self, email: &str) -> Result<(String, UserInfo, String), sqlx::Error> {

        let result = sqlx::query("select password_hash, role::TEXT, name, status::TEXT FROM users WHERE email = $1")
        .bind(email).fetch_one(&self.connection).await?;


        Ok((result.get("password_hash"), UserInfo {
            name: result.get("name"),
            role: result.get("role"),
            email: email.to_string(),
        }, result.get("status")))
    }

    // CATEGORY OPERATIONS
    pub async fn create_category(&self, category: Category) -> Result<(), sqlx::Error> {
        let name = category.name.unwrap_or_default();
        let cat_type = category.category_type.unwrap_or_else(|| "income".to_string());
        let desc = category.description;

        sqlx::query(
            "INSERT INTO categories (name, type, description) VALUES ($1, $2::category_type, $3)"
        )
            .bind(name)
            .bind(cat_type)
            .bind(desc)
            .execute(&self.connection)
            .await?;
        Ok(())
    }

    pub async fn update_category(&self, id: i64, category: Category) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE categories \
             SET name = COALESCE($1, name), \
                 type = COALESCE($2::text::category_type, type), \
                 description = COALESCE($3, description) \
             WHERE id = $4"
        )
            .bind(category.name)
            .bind(category.category_type)
            .bind(category.description)
            .bind(id)
            .execute(&self.connection)
            .await?;
        Ok(())
    }

    pub async fn delete_category(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM categories WHERE id = $1")
            .bind(id)
            .execute(&self.connection)
            .await?;
        Ok(())
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>, sqlx::Error> {
        let records = sqlx::query("SELECT id, name, type::TEXT FROM categories ORDER BY id ASC")
            .fetch_all(&self.connection)
            .await?;

        let categories = records.into_iter().map(|row| Category {
            id: Some(row.get::<i32, _>("id") as i64),
            name: Some(row.get("name")),
            category_type: Some(row.get("type")),
            description: None,
        }).collect();
        Ok(categories)
    }

    pub async fn get_category(&self, id: i64) -> Result<Category, sqlx::Error> {
        let row = sqlx::query("SELECT id, name, type::TEXT, description FROM categories WHERE id = $1")
            .bind(id)
            .fetch_one(&self.connection)
            .await?;

        Ok(Category {
            id: Some(row.get::<i32, _>("id") as i64),
            name: Some(row.get("name")),
            category_type: Some(row.get("type")),
            description: row.try_get("description").unwrap_or(None),
        })
    }

    // COUNTERPARTY OPERATIONS
    pub async fn create_party(&self, party: Party) -> Result<(), sqlx::Error> {
        let name = party.name.unwrap_or_default();
        let p_type = party.party_type.unwrap_or_else(|| "vendor".to_string());
        
        sqlx::query(
            "INSERT INTO counterparties (name, type, email, phone, address) VALUES ($1, $2::counterparty_type, $3, $4, $5)"
        )
            .bind(name)
            .bind(p_type)
            .bind(party.email)
            .bind(party.phone)
            .bind(party.address)
            .execute(&self.connection)
            .await?;
        Ok(())
    }

    pub async fn update_party(&self, id: i64, party: Party) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE counterparties \
             SET name = COALESCE($1, name), \
                 type = COALESCE($2::text::counterparty_type, type), \
                 email = COALESCE($3, email), \
                 phone = COALESCE($4, phone), \
                 address = COALESCE($5, address) \
             WHERE id = $6"
        )
            .bind(party.name)
            .bind(party.party_type)
            .bind(party.email)
            .bind(party.phone)
            .bind(party.address)
            .bind(id)
            .execute(&self.connection)
            .await?;
        Ok(())
    }

    pub async fn delete_party(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM counterparties WHERE id = $1")
            .bind(id)
            .execute(&self.connection)
            .await?;
        Ok(())
    }

    pub async fn get_parties(&self) -> Result<Vec<Party>, sqlx::Error> {
        let records = sqlx::query("SELECT id, name, type::TEXT, email, phone, address FROM counterparties ORDER BY id ASC")
            .fetch_all(&self.connection)
            .await?;

        let parties = records.into_iter().map(|row| Party {
            id: Some(row.get::<i32, _>("id") as i64),
            name: Some(row.get("name")),
            party_type: Some(row.get("type")),
            email: row.try_get("email").unwrap_or(None),
            phone: row.try_get("phone").unwrap_or(None),
            address: row.try_get("address").unwrap_or(None),
        }).collect();
        Ok(parties)
    }

    pub async fn get_party(&self, id: i64) -> Result<Party, sqlx::Error> {
        let row = sqlx::query("SELECT id, name, type::TEXT, email, phone, address FROM counterparties WHERE id = $1")
            .bind(id)
            .fetch_one(&self.connection)
            .await?;

        Ok(Party {
            id: Some(row.get::<i32, _>("id") as i64),
            name: Some(row.get("name")),
            party_type: Some(row.get("type")),
            email: row.try_get("email").unwrap_or(None),
            phone: row.try_get("phone").unwrap_or(None),
            address: row.try_get("address").unwrap_or(None),
        })
    }

    // TRANSACTIONS OPERATIONS
    pub async fn create_transaction(&self, transaction: crate::models::transaction_models::Transaction) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO transactions (amount, transaction_date, status, category_id, counterparty_id, created_by) \
             VALUES ($1, $2, $3::transaction_status, $4, $5, $6)"
        )
            .bind(transaction.amount.unwrap_or(0.0))
            .bind(transaction.transaction_date.unwrap_or_else(|| chrono::Utc::now().naive_utc().date()))
            .bind(transaction.transaction_status.unwrap_or_else(|| "completed".to_string()))
            .bind(transaction.category_id.unwrap_or(1) as i32)
            .bind(transaction.counterparty_id.unwrap_or(1) as i32)
            .bind(transaction.created_by.unwrap_or(1) as i32)
            .execute(&self.connection)
            .await?;
        Ok(())
    }

    pub async fn update_transaction(&self, id: i64, transaction: crate::models::transaction_models::Transaction) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE transactions \
             SET amount = COALESCE($1, amount), \
                 transaction_date = COALESCE($2, transaction_date), \
                 status = COALESCE($3::text::transaction_status, status), \
                 category_id = COALESCE($4, category_id), \
                 counterparty_id = COALESCE($5, counterparty_id), \
                 updated_at = CURRENT_TIMESTAMP \
             WHERE id = $6"
        )
            .bind(transaction.amount)
            .bind(transaction.transaction_date)
            .bind(transaction.transaction_status)
            .bind(transaction.category_id.map(|v| v as i32))
            .bind(transaction.counterparty_id.map(|v| v as i32))
            .bind(id as i32)
            .execute(&self.connection)
            .await?;
        Ok(())
    }

    pub async fn delete_transaction(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM transactions WHERE id = $1")
            .bind(id as i32)
            .execute(&self.connection)
            .await?;
        Ok(())
    }

    pub async fn get_transactions(&self) -> Result<Vec<crate::models::transaction_models::Transaction>, sqlx::Error> {
        let records = sqlx::query("SELECT id, amount, transaction_date, status::TEXT, category_id, counterparty_id, created_by FROM transactions ORDER BY id DESC")
            .fetch_all(&self.connection)
            .await?;

        let transactions = records.into_iter().map(|row| crate::models::transaction_models::Transaction {
            id: Some(row.get::<i32, _>("id") as i64),
            amount: Some(row.get::<f64, _>("amount")),
            transaction_date: Some(row.get::<chrono::NaiveDate, _>("transaction_date")),
            transaction_status: Some(row.get::<String, _>("status")),
            category_id: Some(row.get::<i32, _>("category_id") as i64),
            counterparty_id: Some(row.get::<i32, _>("counterparty_id") as i64),
            created_by: Some(row.get::<i32, _>("created_by") as i64),
        }).collect();
        Ok(transactions)
    }

    pub async fn get_transaction(&self, id: i64) -> Result<crate::models::transaction_models::Transaction, sqlx::Error> {
        let row = sqlx::query("SELECT id, amount, transaction_date, status::TEXT, category_id, counterparty_id, created_by FROM transactions WHERE id = $1")
            .bind(id as i32)
            .fetch_one(&self.connection)
            .await?;

        Ok(crate::models::transaction_models::Transaction {
            id: Some(row.get::<i32, _>("id") as i64),
            amount: Some(row.get::<f64, _>("amount")),
            transaction_date: Some(row.get::<chrono::NaiveDate, _>("transaction_date")),
            transaction_status: Some(row.get::<String, _>("status")),
            category_id: Some(row.get::<i32, _>("category_id") as i64),
            counterparty_id: Some(row.get::<i32, _>("counterparty_id") as i64),
            created_by: Some(row.get::<i32, _>("created_by") as i64),
        })
    }

    // DASHBOARD ANALYTICS
    pub async fn get_recent_dashboard_transactions(&self) -> Result<crate::models::dashboard_models::RecentFeed, sqlx::Error> {
        let records = sqlx::query(
            r#"SELECT t.id, t.amount, TO_CHAR(t.transaction_date, 'YYYY-MM-DD') as date, 
                      COALESCE(c.name, 'Uncategorized') as category, 
                      COALESCE(c.type::TEXT, 'expenses') as type, 
                      COALESCE(p.name, 'Unknown') as counterparty, 
                      t.status::TEXT as status 
               FROM transactions t 
               LEFT JOIN categories c ON t.category_id = c.id 
               LEFT JOIN counterparties p ON t.counterparty_id = p.id 
               ORDER BY t.transaction_date DESC, t.id DESC 
               LIMIT 10"#
        ).fetch_all(&self.connection).await?;

        let transactions = records.into_iter().map(|row| {
            crate::models::dashboard_models::RecentTransaction {
                id: row.get::<i32, _>("id") as i64,
                amount: row.get("amount"),
                date: row.get("date"),
                category: row.get("category"),
                r#type: row.get("type"),
                counterparty: row.get("counterparty"),
                status: row.get("status"),
            }
        }).collect();

        Ok(crate::models::dashboard_models::RecentFeed { transactions })
    }

    pub async fn get_dashboard_summary(&self) -> Result<crate::models::dashboard_models::DashboardSummary, sqlx::Error> {
        let record = sqlx::query(
            r#"SELECT 
                 COALESCE(SUM(t.amount) FILTER (WHERE c.type = 'income'), 0.0) as total_income,
                 COALESCE(SUM(t.amount) FILTER (WHERE c.type = 'expenses'), 0.0) as total_expenses,
                 COUNT(t.id) as total_transactions
               FROM transactions t 
               LEFT JOIN categories c ON t.category_id = c.id"#
        ).fetch_one(&self.connection).await?;

        let total_income: f64 = record.get("total_income");
        let total_expenses: f64 = record.get("total_expenses");

        Ok(crate::models::dashboard_models::DashboardSummary {
            total_income,
            total_expenses,
            net_balance: total_income - total_expenses,
            total_transactions: record.get::<i64, _>("total_transactions"),
        })
    }

    pub async fn get_dashboard_categories(&self) -> Result<crate::models::dashboard_models::CategoryAnalytics, sqlx::Error> {
        let records = sqlx::query(
            r#"SELECT c.name as category, c.type::TEXT as type, SUM(t.amount) as total 
               FROM transactions t 
               JOIN categories c ON t.category_id = c.id 
               GROUP BY c.name, c.type"#
        ).fetch_all(&self.connection).await?;

        let mut expense_categories = Vec::new();
        let mut income_categories = Vec::new();

        for row in records {
            let cat_type: String = row.get("type");
            let item = crate::models::dashboard_models::CategoryTotal {
                category: row.get("category"),
                total: row.get("total"),
            };
            if cat_type == "income" {
                income_categories.push(item);
            } else {
                expense_categories.push(item);
            }
        }

        Ok(crate::models::dashboard_models::CategoryAnalytics {
            expense_categories,
            income_categories,
        })
    }

    pub async fn get_dashboard_trends(&self, year: i32) -> Result<crate::models::dashboard_models::TrendAnalytics, sqlx::Error> {
        let records = sqlx::query(
            r#"SELECT 
                 TRIM(TO_CHAR(t.transaction_date, 'Month')) as month,
                 CAST(EXTRACT(MONTH FROM t.transaction_date) AS INTEGER) as month_num,
                 COALESCE(SUM(t.amount) FILTER (WHERE c.type = 'income'), 0.0) as income,
                 COALESCE(SUM(t.amount) FILTER (WHERE c.type = 'expenses'), 0.0) as expenses
               FROM transactions t 
               JOIN categories c ON t.category_id = c.id 
               WHERE EXTRACT(YEAR FROM t.transaction_date) = $1::double precision 
               GROUP BY month, month_num 
               ORDER BY month_num"#
        ).bind(year as f64).fetch_all(&self.connection).await?;

        let months = records.into_iter().map(|row| {
            let income: f64 = row.get("income");
            let expenses: f64 = row.get("expenses");
            crate::models::dashboard_models::MonthTrend {
                month: row.get("month"),
                income,
                expenses,
                net: income - expenses,
            }
        }).collect();

        Ok(crate::models::dashboard_models::TrendAnalytics {
            year,
            months,
        })
    }
}