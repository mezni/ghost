CREATE TABLE global_by_minute
(
    id NUMBER,
    ts_id NUMBER,
    score VARCHAR(40),
    category VARCHAR(40),
    network_type VARCHAR(40),
    device_type VARCHAR(40),
    throughput_up DOUBLE PRECISION,
    throughput_down DOUBLE PRECISION,    
    throughput_up DOUBLE PRECISION,
    throughput_down DOUBLE PRECISION,    
    latency_up DOUBLE PRECISION,
    latency_down DOUBLE PRECISION,    
    packet_loss_up DOUBLE PRECISION,
    packet_loss_down DOUBLE PRECISION,  
    subscribers_count INT,
    sessions_count INT,
    PRIMARY KEY (id)

)