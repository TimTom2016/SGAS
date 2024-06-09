-- Add up migration script here
CREATE TABLE graph(
    id INT NOT NULL PRIMARY KEY,
    sensor_id INT NOT NULL,
    graph_type INT NOT NULL,
    FOREIGN KEY(sensor_id) REFERENCES sensor(sensorId)
);

CREATE TABLE favorites(
    graph INT NOT NULL,
    user_id INT NOT NULL,
    x INT NOT NULL,
    y INT NOT NULL,
    FOREIGN KEY(graph) REFERENCES graph(id),
    FOREIGN KEY(user_id) REFERENCES users(id),

    PRIMARY KEY(user_id,x,y) 
)