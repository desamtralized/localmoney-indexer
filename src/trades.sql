CREATE TABLE denoms (
    id INT PRIMARY KEY AUTO_INCREMENT,
    native VARCHAR(20) NOT NULL
);

CREATE TABLE trades (
    id INT PRIMARY KEY AUTO_INCREMENT,
    addr VARCHAR(100) NOT NULL,
    buyer VARCHAR(100) NOT NULL,
    buyer_contact TEXT NOT NULL,
    buyer_encryption_key TEXT NOT NULL,
    seller VARCHAR(100) NOT NULL,
    seller_contact TEXT,
    seller_encryption_key TEXT NOT NULL,
    arbitrator VARCHAR(100),
    arbitrator_encryption_key TEXT,
    arbitrator_seller_contact TEXT,
    arbitrator_buyer_contact TEXT,
    offer_contract VARCHAR(100) NOT NULL,
    offer_id INT NOT NULL,
    created_at INT NOT NULL,
    expires_at INT NOT NULL,
    enables_dispute_at INT,
    amount VARCHAR(50) NOT NULL,
    fiat VARCHAR(20) NOT NULL,
    denom_fiat_price VARCHAR(20) NOT NULL,
    state VARCHAR(50) NOT NULL
);

CREATE TABLE state_history (
    id INT PRIMARY KEY AUTO_INCREMENT,
    trade_id INT NOT NULL,
    actor VARCHAR(100) NOT NULL,
    state VARCHAR(50) NOT NULL,
    timestamp INT NOT NULL
);

CREATE TABLE offer (
    id INT PRIMARY KEY AUTO_INCREMENT,
    owner VARCHAR(100) NOT NULL,
    offer_type VARCHAR(20) NOT NULL,
    fiat_currency VARCHAR(20) NOT NULL,
    rate VARCHAR(20) NOT NULL,
    min_amount VARCHAR(50) NOT NULL,
    max_amount VARCHAR(50) NOT NULL,
    description TEXT,
    state VARCHAR(50) NOT NULL,
    timestamp INT NOT NULL
);

CREATE TABLE offer_denom (
    id INT PRIMARY KEY AUTO_INCREMENT,
    offer_id INT NOT NULL,
    denom_id INT NOT NULL,
    FOREIGN KEY (offer_id) REFERENCES offer (id) ON DELETE CASCADE,
    FOREIGN KEY (denom_id) REFERENCES denoms (id) ON DELETE CASCADE
);

CREATE TABLE trade_denom (
    id INT PRIMARY KEY AUTO_INCREMENT,
    trade_id INT NOT NULL,
    denom_id INT NOT NULL,
    FOREIGN KEY (trade_id) REFERENCES trades (id) ON DELETE CASCADE,
    FOREIGN KEY (denom_id) REFERENCES denoms (id) ON DELETE CASCADE
);
