# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: server.proto
# Protobuf Python Version: 5.26.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x0cserver.proto\x12\x04sgas\"j\n\x16\x61\x64\x64_new_sensor_message\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\x0c\n\x04type\x18\x02 \x01(\t\x12\x10\n\x03pin\x18\x03 \x01(\x05H\x00\x88\x01\x01\x12\x11\n\x04\x61\x64\x64r\x18\x04 \x01(\tH\x01\x88\x01\x01\x42\x06\n\x04_pinB\x07\n\x05_addr\"\x0e\n\x0c\x64one_message\")\n\x15\x64\x65lete_sensor_message\x12\x10\n\x08sensorId\x18\x01 \x01(\x05\x32\xa4\x01\n\x0csgas_service\x12H\n\x12new_sensor_request\x12\x1c.sgas.add_new_sensor_message\x1a\x12.sgas.done_message\"\x00\x12J\n\x15\x64\x65lete_sensor_request\x12\x1b.sgas.delete_sensor_message\x1a\x12.sgas.done_message\"\x00\x62\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'server_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  DESCRIPTOR._loaded_options = None
  _globals['_ADD_NEW_SENSOR_MESSAGE']._serialized_start=22
  _globals['_ADD_NEW_SENSOR_MESSAGE']._serialized_end=128
  _globals['_DONE_MESSAGE']._serialized_start=130
  _globals['_DONE_MESSAGE']._serialized_end=144
  _globals['_DELETE_SENSOR_MESSAGE']._serialized_start=146
  _globals['_DELETE_SENSOR_MESSAGE']._serialized_end=187
  _globals['_SGAS_SERVICE']._serialized_start=190
  _globals['_SGAS_SERVICE']._serialized_end=354
# @@protoc_insertion_point(module_scope)