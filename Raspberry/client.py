"""
Dominik Fiegl 06.2024
Test Client file to test the gRPC server.
Not needed anymore 
"""
import grpc
import server_pb2
import server_pb2_grpc
import time
def run():
    # Open a gRPC channel
    with grpc.insecure_channel('localhost:50051') as channel:

        # Create a stub (client)
        stub = server_pb2_grpc.sgas_serviceStub(channel)

        # Create a new sensor request
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Taster', type='GPIO', pin=4))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(1)
        _type=stub.get_supported_sensor_types(server_pb2.supported_sensor_types_message())
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Licht_Sensor', type=_type.supported_sensor_types[0]))
        print("i2c added")
        time.sleep(60)
        # Delete a sensor request
        stub.delete_sensor_request(server_pb2.delete_sensor_message(sensorId=1))  
        stub.delete_sensor_request(server_pb2.delete_sensor_message(sensorId=2))
        print("Deleted sensor") # + response.message) need to add variable tp done_message in server.proto

if __name__ == '__main__':
    run()