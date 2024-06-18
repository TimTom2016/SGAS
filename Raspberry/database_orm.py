#orm-mapper library
from peewee import *
import datetime


database = MySQLDatabase('sgas',
                                    user='python',
                                    password='python',
                                    host='localhost',
                                    )


class sensor(Model):
    sensorID = AutoField(primary_key=True)
    name = CharField()
    type = CharField(default= 'GPIO')
    pin = CharField(default= None)
    addr = CharField(default= None)
    
    class Meta:
        database = database

class sensorValue(Model):
    sensorID = ForeignKeyField(sensor, backref='values')
    value = FloatField()
    timestamp = DateTimeField()

    class Meta:
        database = database
