CREATE DATABASE IF NOT EXISTS clients_rust_db;
use clients_rust_db;

CREATE TABLE IF NOT EXISTS clients (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    phone VARCHAR(20)
);

INSERT INTO clients (name, phone) VALUES
('Danilo', '11 9999-9991'),
('sheila', '11 2222-2222'),
('Junes', '11 3333-3333');