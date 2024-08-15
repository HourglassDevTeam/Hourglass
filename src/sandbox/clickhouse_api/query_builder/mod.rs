#[allow(dead_code)]
pub struct ClickHouseQueryBuilder {
    select_clause: String,
    from_clause: String,
    where_clause: Option<String>,
    order_by_clause: Option<String>,
    order_direction: Option<String>,      // 存储排序方向（ASC 或 DESC）
    limit_clause: Option<String>,
    offset_clause: Option<String>, // Add this line
}

#[allow(dead_code)]
impl ClickHouseQueryBuilder {
    // 初始化构造器
    pub fn new() -> Self {
        Self {
            select_clause: String::new(),
            from_clause: String::new(),
            where_clause: None,
            order_by_clause: None,
            order_direction: None,
            limit_clause: None,
            offset_clause: None,
        }
    }

    // 设置SELECT子句
    pub fn select(mut self, fields: &str) -> Self {
        self.select_clause = format!("SELECT {}", fields);
        self
    }

    // 设置FROM子句
    pub fn from(mut self, table: &str) -> Self {
        self.from_clause = format!("FROM {}", table);
        self
    }

    // 添加WHERE条件
    pub fn where_clause(mut self, condition: &str) -> Self {
        self.where_clause = Some(format!("WHERE {}", condition));
        self
    }
    // 添加LIKE条件
    pub fn like_clause(mut self, field: &str, pattern: &str) -> Self {
        self.where_clause = Some(self.where_clause.map_or_else(
            || format!("WHERE {} LIKE '{}'", field, pattern),
            |existing_clause| format!("{} AND {} LIKE '{}'", existing_clause, field, pattern),
        ));
        self
    }

    // 添加NOT LIKE条件
    pub fn not_like_clause(mut self, field: &str, pattern: &str) -> Self {
        self.where_clause = Some(self.where_clause.map_or_else(
            || format!("WHERE {} NOT LIKE '{}'", field, pattern),
            |existing_clause| format!("{} AND {} NOT LIKE '{}'", existing_clause, field, pattern),
        ));
        self
    }
    // 添加ORDER BY子句
    pub fn order(mut self, field: &str, direction: Option<&str>) -> Self {
        match direction {
            Some("ASC") | Some("DESC") => {
                self.order_by_clause = Some(field.to_owned());
                // 存储排序方向，不需要解引用
                self.order_direction = direction.map(|d| d.to_owned());
            }
            None => {
                // 如果不需要排序，则设置为 None
                self.order_by_clause = None;
                self.order_direction = None;
            }
            _ => {println!("Direction must be 'ASC', 'DESC', or None")}
        }
        self
    }
    // 添加LIMIT子句
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit_clause = Some(format!("LIMIT {}", limit));
        self
    }
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset_clause = Some(format!("OFFSET {}", offset));
        self
    }

    // 构建最终的查询
    pub fn build(self) -> String {
        let mut query = format!("{} {}", self.select_clause, self.from_clause);

        if let Some(where_clause) = self.where_clause {
            query.push_str(&format!(" {}", where_clause));
        }

        if let Some(order_by_clause) = self.order_by_clause {
            query.push_str(&format!(" {}", order_by_clause));
        }

        if let Some(limit_clause) = self.limit_clause {
            query.push_str(&format!(" {}", limit_clause));
        }

        query
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_query() {
        let query = ClickHouseQueryBuilder::new()
            .select("id, name")
            .from("users")
            .build();
        assert_eq!(query, "SELECT id, name FROM users");
    }

    #[test]
    fn test_where_query() {
        let query = ClickHouseQueryBuilder::new()
            .select("*")
            .from("users")
            .where_clause("id = 1")
            .build();
        assert_eq!(query, "SELECT * FROM users WHERE id = 1");
    }

    #[test]
    fn test_like_query() {
        let query = ClickHouseQueryBuilder::new()
            .select("*")
            .from("users")
            .like_clause("name", "%example%")
            .build();
        assert_eq!(query, "SELECT * FROM users WHERE name LIKE '%example%'");
    }

    #[test]
    fn test_not_like_query() {
        let query = ClickHouseQueryBuilder::new()
            .select("*")
            .from("products")
            .not_like_clause("description", "%old%")
            .build();
        assert_eq!(query, "SELECT * FROM products WHERE description NOT LIKE '%old%'");
    }

    #[test]
    fn test_combined_query() {
        let query = ClickHouseQueryBuilder::new()
            .select("id, name")
            .from("users")
            .where_clause("age > 18")
            .like_clause("email", "%@mail.com")
            .order_by("created_at DESC")
            .limit(10)
            .build();
        assert_eq!(query, "SELECT id, name FROM users WHERE age > 18 AND email LIKE '%@mail.com' ORDER BY created_at DESC LIMIT 10");
    }

    #[test]
    fn test_query_with_multiple_conditions() {
        let query = ClickHouseQueryBuilder::new()
            .select("*")
            .from("users")
            .where_clause("id = 1")
            .like_clause("username", "%user%")
            .not_like_clause("password", "%weak%")
            .build();
        assert_eq!(query, "SELECT * FROM users WHERE id = 1 AND username LIKE '%user%' AND password NOT LIKE '%weak%'");
    }
}
