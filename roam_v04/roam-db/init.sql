CREATE TABLE networks (
    id SERIAL PRIMARY KEY,
    plmn VARCHAR(10) NOT NULL,
    mcc VARCHAR(3) NOT NULL,
    mnc VARCHAR(3) NOT NULL,
    tadig VARCHAR(10) NOT NULL,
    created_by VARCHAR(100) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE technologies (
    id SERIAL PRIMARY KEY,
    technology VARCHAR(50) NOT NULL UNIQUE,
    created_by VARCHAR(100) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Junction table for many-to-many relationship
CREATE TABLE network_technologies (
    network_id INTEGER REFERENCES networks(id) ON DELETE CASCADE,
    technology_id INTEGER REFERENCES technologies(id) ON DELETE CASCADE,
    created_by VARCHAR(100) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_by VARCHAR(100) NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (network_id, technology_id)
);