# Schema:
```sql
CREATE TABLE Manufacturers (
    manufacturer_id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    contact_info VARCHAR(255)
);
```

# API Endpoints
| HTTP Method | ASCII                         | Description                          |
|-------------|-------------------------------|--------------------------------------|
|POST         | /manufacturers                | Create a new manufacturer.           |
|GET          | /manufacturers                | Get all manufacturers.               |
|GET          | /manufacturers/{id}           | Get a single manufacturer by ID.     |
|PUT          | /manufacturers/{id}           | Update a manufacturer by ID.         |
|DELETE       | /manufacturers/{id}           | Delete a manufacturer by ID.         |

# TO DO
 - Add error handling for database operations.
 - Use connection pooling for better performance.
 - Add authentication and authorization.