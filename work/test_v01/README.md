# Schema:
```sql
CREATE TABLE Manufacturers (
    manufacturer_id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    contact_info VARCHAR(255)
);
```

# TO DO
 - Add error handling for database operations.
 - Use connection pooling for better performance.
 - Add authentication and authorization.