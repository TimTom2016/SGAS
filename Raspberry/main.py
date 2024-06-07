import database_orm as db

import peewee as pw
import datetime
import time

#GPIO setup
import RPi.GPIO as GPIO


class GPIOsensor():
    def __init__(self, name, pin, parentSensorList):
        self.name = name
        self.pin = pin

        #append the sensor to the list of sensors
        parentSensorList.append(self)
        #save the sensor to the database
        temp = db.sensor(name=self.name,type='GPIO', pin=self.pin)
        temp.save()

        #specific setup
        GPIO.setmode(GPIO.BCM)
        GPIO.setup(self.pin, GPIO.IN)
    
    def getinf(self):
        return (self.id, GPIO.input(self.pin), datetime.datetime.now())


def update_sensors_to_db():
    for item in sensorList:
        #create a row in the sensorValue table and saves it
        dbwrite = db.sensorValue(item.getinf())
        dbwrite.save()


sensorList = []

try:
    # Connect to the database
    db.database.connect()
    # Retrieve the sensor values from the database
    initialSensors = db.Sensor.select()
    # Close the database connection
    for sensor in initialSensors:
        # Create a new sensor object
        GPIOsensor(sensor.name, sensor.pin, sensorList)
        # Add the sensor object to the list of sensors

    test = 0
    while(True):
        if new == 1:  #push by the webserver that new sensors are here
            pin=4   #send by grpc
            name = "test" #send by grpc
            #create a new sensor object
            GPIOsensor(name, pin, sensorList)
            test += 1

        update_sensors_to_db()
        time.sleep(10)

except(KeyboardInterrupt):
    print("Interrupted by user. Shutting down...")
    GPIO.cleanup()
    db.close()