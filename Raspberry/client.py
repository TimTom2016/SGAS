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
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        stub.new_sensor_request(server_pb2.add_new_sensor_message(name='Sensor1', type='GPIO', pin=1))
        print("New sensor added") # + response.message) need to add variable tp done_message in server.proto
        time.sleep(0.01)
        time.sleep(1)
        # Delete a sensor request
        response = stub.delete_sensor_request(server_pb2.delete_sensor_message(sensorId=1))
        print("Deleted sensor") # + response.message) need to add variable tp done_message in server.proto

if __name__ == '__main__':
    run()