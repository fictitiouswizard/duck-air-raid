CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE library_system (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    main_branch_id UUID
);

CREATE TABLE library (
     id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
     name VARCHAR(255) NOT NULL,
     slug VARCHAR(255) UNIQUE NOT NULL,
     street_address VARCHAR(255) NOT NULL,
     city VARCHAR(255) NOT NULL,
     state_or_province VARCHAR(255) NOT NULL,
     postal_code VARCHAR(255) NOT NULL,
     point_balance BIGINT NOT NULL,
     library_system_id UUID REFERENCES library_system(id)
);

CREATE TABLE quest (
   id VARCHAR(255) PRIMARY KEY DEFAULT uuid_generate_v4(),
   library_id UUID REFERENCES library(id),
   book_name VARCHAR(255) NOT NULL,
   author VARCHAR(255) NOT NULL,
   book_series VARCHAR(255),
   point_value BIGINT NOT NULL
);