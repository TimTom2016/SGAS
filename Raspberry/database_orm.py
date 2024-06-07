#orm-mapper library
from peewee import *
import datetime


database = MySQLDatabase('sgas',
                                    user='root',
                                    password='',
                                    host='127.0.0.1'
                                    )


class sensor(Model):
    id = AutoField(primary_key=True)
    name = CharField()
    type = CharField(default= 'GPIO')
    pin = CharField(default= None)
    address = CharField(default= None)
    
    class Meta:
        database = database

class sensorValue(Model):
    deviceID = ForeignKeyField(sensor, backref='values')
    value = FloatField()
    timestamp = DateTimeField(default=datetime.datetime.now)

    class Meta:
        database = database
