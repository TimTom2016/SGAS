from concurrent import futures
import grpc
import logging
import server_pb2_grpc
import server_pb2  # Uncomment this if you're using server_pb2.done_message()

class sgas_serviceServicer(server_pb2_grpc.sgas_serviceServicer):
    def new_sensor_request(self, request, context):
        _name = request.name
        _type = request.type
        _pin = request.pin
        _addr = request.addr
        print(_name, _type, _pin, _addr)
        # temp = db.Sensor(name=_name, type=_type, pin=_pin, addr=_addr)
        # temp.save()
        return server_pb2.done_message()

    def delete_sensor_request(self, request, context):
        _sensorId = request.sensorId
        print(_sensorId)
        # sensor = db.Sensor.get(db.Sensor.sensorId == _sensorId)
        # sensor.delete_instance(recursive=True)
        return server_pb2.done_message()

def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    server_pb2_grpc.add_sgas_serviceServicer_to_server(sgas_serviceServicer(), server)
    server.add_insecure_port('[::]:50051')
    server.start()
    server.wait_for_termination()

if __name__ == "__main__":
    logging.basicConfig()
    serve()