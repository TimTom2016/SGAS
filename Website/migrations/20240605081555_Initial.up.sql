-- Add up migration script here
-- create table device(
--     deviceId INT PRIMARY KEY AUTO_INCREMENT,
--     name VARCHAR(32) NOT NULL,
--     macAddr VARCHAR(37)
-- );

-- table
create table sensor(
    sensorId INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(32) NOT NULL,
    type VARCHAR(32) NOT NULL,
    pin INT,
    addr VARCHAR(32)
);

-- table
CREATE TABLE sensorValue (
    time_stamp TIMESTAMP NOT NULL,
    sensorId_id INT NOT NULL,
    value FLOAT NOT NULL,
    PRIMARY KEY (sensorId_id, time_stamp),
    FOREIGN KEY (sensorId_id) REFERENCES sensor(sensorId)
);

-- relation
create table canSee(
    sensorId INT NOT NULL,
    id INT NOT NULL,
    FOREIGN KEY (sensorId) REFERENCES sensor(sensorId),
    FOREIGN KEY (id) REFERENCES users(id),
    PRIMARY KEY (sensorId, id)
);

-- relation
-- create table isOn(
--     deviceId INT NOT NULL,
--     sensorId INT NOT NULL,
--     FOREIGN KEY (deviceId) REFERENCES device(deviceId),
--     FOREIGN KEY (sensorId) REFERENCES sensor(sensorId),
--     PRIMARY KEY (deviceId, sensorId)
-- );