#orm-mapper library
from peewee import *
import datetime


database = MySQLDatabase('sgas',
                                    user='python',
                                    password='python',
                                    host='127.0.0.1',
                                    )

class sensor(Model):
    sensorId = AutoField(primary_key=True)
    name = CharField()
    type = CharField(default= 'GPIO')
    pin = CharField(default= None)
    addr = CharField(default= None)
    
    class Meta:
        database = database
        table_name = 'sensor'

class sensorValue(Model):
    sensorId = ForeignKeyField(sensor, backref='values')
    value = FloatField()
    time_stamp = DateTimeField(default=datetime.datetime.now)

    class Meta:
        database = database
        table_name = 'sensorValue'
