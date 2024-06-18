"""
Dominik Fiegl 06.2024
Program to start the gRPC server based on the server.proto file.
The server listens on port 50051 for incoming requests.
Is meant to be implemented to the main file of the SGAS project:
logging.basicConfig()
server = serve()
threading.Thread(target=server.wait_for_termination).start()
"""
from concurrent import futures
import grpc
import logging
import server_pb2_grpc
import server_pb2  # Uncomment this line if you're using server_pb2.done_message()
import threading

sensorList = []

class sgas_serviceServicer(server_pb2_grpc.sgas_serviceServicer):
    def new_sensor_request(self, request, context):
        _name = request.name
        _type = request.type
        _pin = request.pin
        _addr = request.addr
        #call function to create new sensor
        if _type == 'GPIO':
            GPIOsensor(_name, _pin, sensorList)
        else:
            I2Csensor(_name, _addr, _type, sensorList)
        
        return server_pb2.done_message()

    def delete_sensor_request(self, request, context):
        _sensorId = request.sensorId
        #call function to delete sensor
        sensorList[_sensorId].delete()
        return server_pb2.done_message()

def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=1))
    server_pb2_grpc.add_sgas_serviceServicer_to_server(sgas_serviceServicer(), server)
    server.add_insecure_port('[::]:50051')
    server.start()
    return server

#starts the gRPC stuff
logging.basicConfig()
server = serve()
threading.Thread(target=server.wait_for_termination).start()



"""
Nico Zehetner 13.06.2024
"""


import database_orm as db

import peewee as pw
import datetime
import time

#GPIO setup
import RPi.GPIO as GPIO
#TSL2591 setup
import board
import adafruit_tsl2591 as luxSensor


class GPIOsensor():
    def __init__(self, name, pin, parentSensorList):
        self.name = name
        self.pin = int(pin)
        self.ParentSensorList = parentSensorList

        #append the sensor to the list of sensors
        self.parentSensorList.append(self)
        #save the sensor to the database
        temp = db.sensor(name=self.name,type='GPIO', pin=self.pin, address=None)
        temp.save()

        #specific setup
        GPIO.setmode(GPIO.BCM)
        GPIO.setup(self.pin, GPIO.IN)
    
    
    def getval(self):
        return GPIO.input(self.pin)
    
    def delete(self):
        db.sensor.get(db.sensor.name == self.name).delete_instance()
        self.parentSensorList.remove(self)

class I2Csensor():
    def __init__(self, name, addr, type, parentSensorList):
        self.name = name
        self.addr = addr
        self.type = type 
        self.ParentSensorList = parentSensorList

        #append the sensor to the list of sensors
        self.parentSensorList.append(self)
        #save the sensor to the database
        temp = db.sensor(name=self.name, type=self.type, pin=None, address=self.addr)
        temp.save()

        #specific setup
        if self.type == 'TSL2591':
            sensor = luxSensor.TSL2591(board.i2c)

    def getval(self):
        if self.type == 'TSL2591':
            value = sensor.lux()
        else:
            value = 0
        return value
    
    def delete(self):
        db.sensor.get(db.sensor.name == self.name).delete_instance()
        self.parentSensorList.remove(self)



def update_sensors_to_db():
    for item in sensorList:
        #create a row in the sensorValue table and saves it
        dbwrite = db.sensorValue(item.getval())
        dbwrite.save()


try:
    #--------------------------------
    #initial setup
    #--------------------------------
    
    # Connect to the database
    db.database.connect()
    # Retrieve the sensor values from the database
    initialSensors = db.Sensor.select()
    # Close the database connection
    for sensor in initialSensors:
        # Create a new sensor object
        GPIOsensor(sensor.name, sensor.pin, sensorList)
        # Add the sensor object to the list of sensors

    #--------------------------------
    #main loop
    #--------------------------------
    while(True):
        update_sensors_to_db()
        time.sleep(10)

except(KeyboardInterrupt):
    print("Interrupted by user. Shutting down...")
    GPIO.cleanup()
    db.close()