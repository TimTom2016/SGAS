-- check if 1 to n does change the relation

-- table
create table device(
    deviceId INT AUTO_INCREMENT PRIMARY KEY,
    name STRING(32) NOT NULL,
    macAddr STRING(37) NOT NULL,
);

-- table
create table sensor(
    sensorId INT AUTO_INCREMENT PRIMARY KEY,
    name STRING(32) NOT NULL,
    type STRING(32) NOT NULL,
    pin INT,
    addr STRING(32)
);

-- table
create table sensorValue(
    timeStamp TIMESTAMP PRIMARY KEY,
    value FLOAT NOT NULL
);

-- relation
create table canSee(
    deviceId INT NOT NULL AUTO_INCREMENT,
    id INT NOT NULL AUTO_INCREMENT,
    FOREIGN KEY (deviceId) REFERENCES device(deviceId),
    FOREIGN KEY (user) REFERENCES users(id),
    PRIMARY KEY (deviceId, id)
);

-- relation
create table isOn(
    deviceId INT NOT NULL AUTO_INCREMENT,
    sensorId INT NOT NULL AUTO_INCREMENT,
    FOREIGN KEY (deviceId) REFERENCES device(deviceId),
    FOREIGN KEY (sensorId) REFERENCES sensor(sensorId),
    PRIMARY KEY (deviceId, sensorId)
);

-- relation
create table measures(
    timeStamp TIMESTAMP NOT NULL,
    sensorId INT NOT NULL AUTO_INCREMENT,
    FOREIGN KEY (sensorId) REFERENCES sensor(sensorId),
    FOREIGN KEY (timeStamp) REFERENCES sensorValue(timeStamp),
);