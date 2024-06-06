-- Add up migration script here
CREATE TABLE graph_definition (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    options JSON NOT NULL,
    user_id INT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE graph_sensor(
    sensor_id INT NOT NULL,
    graph_definition INT NOT NULL,
    FOREIGN KEY (sensor_id) REFERENCES sensor(sensorId),
    FOREIGN KEY (graph_definition) REFERENCES graph_definition(id),
    PRIMARY KEY (sensor_id,graph_definition)
)