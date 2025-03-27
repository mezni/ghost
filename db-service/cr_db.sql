CREATE TABLE IF NOT EXISTS batch_execs (
    id SERIAL PRIMARY KEY,
    batch_name VARCHAR(100) NOT NULL,
    start_time TIMESTAMP,
    end_time TIMESTAMP,        
    batch_status VARCHAR(10) 
);
