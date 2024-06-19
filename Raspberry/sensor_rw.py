import datetime
import time
import threading
import logging
from concurrent import futures
from typing import Optional
import server_pb2_grpc
import server_pb2


import grpc
import RPi.GPIO as GPIO
import board
from python_tsl2591 import tsl2591
import database_orm as db

# Configure logging
logging.basicConfig(level=logging.INFO)

# List to store sensor instances
sensorList = []
init_state = True
rate=1
GPIO.setmode(GPIO.BCM)

class SGASServiceServicer(server_pb2_grpc.sgas_serviceServicer):
    def new_sensor_request(self, request, context):
        """ gRPC call handler for adding a new sensor """
        _name = request.name
        _type = request.type
        _pin = request.pin
        _addr = request.addr
        Sensors(name=_name, pin=_pin, sensorId=None, type=_type, addr=_addr)
        return server_pb2.done_message()

    def delete_sensor_request(self, request, context):
        """ gRPC call handler for deleting a sensor """
        _sensorId = request.sensorId
        try:
            db.sensorValue.delete().where(db.sensorValue.sensorId_id == _sensorId).execute()
            db.sensor.delete().where(db.sensor.sensorId == _sensorId).execute()

            # Remove sensor instance from sensorList
            for sensor in sensorList:
                if sensor.sensorId == _sensorId:
                    sensor.remove_from_list()
                    break

        except Exception as error:
            logging.error(f"Failed to delete sensor with ID {_sensorId}: {error}")
        return server_pb2.done_message()

    def get_supported_sensor_types (self, request, context):
        """ gRPC call handler for getting supported sensor types """
        types=['TSL2591']
        return server_pb2.supported_sensor_types_message(supported_sensor_types=types)

def serve():
    """ Function to start the gRPC server """
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=1))
    server_pb2_grpc.add_sgas_serviceServicer_to_server(SGASServiceServicer(), server)
    server.add_insecure_port('[::]:50051')
    server.start()
    return server

class Sensors:
    def __init__(self, name: str, pin: int, sensorId: None, type: str = 'GPIO', addr: str = '', ParentsensorList=sensorList):
        """ Sensor class for managing sensor instances """
        self.name = name
        self.pin = pin
        self.type = type
        self.addr = addr
        ParentsensorList.append(self)
        self.sensorId = self.add_to_db(sensorId)

        if type == 'GPIO':
            GPIO.setup(int(self.pin), GPIO.IN)
        elif type == 'TSL2591':
            board.I2C()
            self.sensor = tsl2591()

    def add_to_db(self, sensorId: None):
        """ Method to add sensor to the database """
        if not init_state:
            temp = db.sensor(name=self.name, type=self.type, pin=self.pin, addr=self.addr)
            try:
                temp.save()
            except Exception as error:
                logging.error(f"Failed to save sensor {self.name} to database: {error}")
                return None
            return temp.sensorId
        else:
            return sensorId

    def get_type(self):
        """ Method to get sensor type """
        return self.type

    def get_values_GPIO(self):
        """ Method to get GPIO sensor values """
        return self.sensorId, GPIO.input(int(self.pin)), datetime.datetime.now()

    def get_values_TSL2591(self):
        """ Method to get I2C sensor values """
        full, ir = self.sensor.get_full_luminosity()  # Read raw values (full spectrum and infared spectrum).
        lux = self.sensor.calculate_lux(full, ir)  # Convert raw values to Lux.
        return self.sensorId, lux, datetime.datetime.now()

    def remove_from_list(self):
        """ Method to remove sensor instance from sensorList """
        if self in sensorList:
            sensorList.remove(self)

    # Add functionality for receiving data from IP sources if needed

def update_sensors_to_db():
    """ Function to update sensor values in the database """
    for item in sensorList:
        if item.get_type() == 'GPIO':
            info = item.get_values_GPIO()
        elif item.get_type() == 'TSL2591':
            info = item.get_values_TSL2591()
        else:
            # Add functionality for receiving data from IP sources if needed
            continue
        try:
            db.sensorValue(sensorId=info[0], value=info[1], time_stamp=info[2]).save()
        except Exception as error:
            logging.error(f"Failed to save sensor value to database: {error}")

try:
    # Initialize GPIO and database connection
    db.database.connect()
    
    # Retrieve and create sensor objects from the database
    initialSensors = db.sensor.select().order_by(db.sensor.sensorId)
    for sensor in initialSensors:
        Sensors(name=sensor.name, pin=sensor.pin, sensorId=sensor.sensorId, type=sensor.type, addr=sensor.addr)
    
    # Set initialization state
    init_state = False
    logging.info("init done... Starting gRPC")
    # Start gRPC server in a separate thread
    server = serve()
    server_thread = threading.Thread(target=server.wait_for_termination)
    server_thread.start()

    # Continuously update sensor values in the database
    while True:
        update_sensors_to_db()
        time.sleep(rate)

except KeyboardInterrupt:
    logging.info("Interrupted by user. Shutting down...")
finally:
    # Clean up GPIO and close database connection
    GPIO.cleanup()
    db.database.close()
    
    # Stop gRPC server if it's running
    if 'server' in locals():
        server.stop(0)
    
    # Wait for server thread to join if it's still alive
    if 'server_thread' in locals() and server_thread.is_alive():
        server_thread.join()