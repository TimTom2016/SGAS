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

class sgas_serviceServicer(server_pb2_grpc.sgas_serviceServicer):
    def new_sensor_request(self, request, context):
        _name = request.name
        _type = request.type
        _pin = request.pin
        _addr = request.addr
        #call function to create new sensor
        return server_pb2.done_message()

    def delete_sensor_request(self, request, context):
        _sensorId = request.sensorId
        #call function to delete sensor
        return server_pb2.done_message()

def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=1))
    server_pb2_grpc.add_sgas_serviceServicer_to_server(sgas_serviceServicer(), server)
    server.add_insecure_port('[::]:50051')
    server.start()
    return server

#only runs if it is the main file (not imported)
if __name__ == "__main__":
    import time
    logging.basicConfig()
    server = serve()
    print("Server started")
    print("Server listening on port 50051")
    threading.Thread(target=server.wait_for_termination).start()
    print("Continuing with the rest of the Python file...")
    while True:
        time.sleep(1)
        print("Server still running...")
        pass