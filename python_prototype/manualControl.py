import RPi.GPIO as GPIO

SER_PIN = 4
RSK_PIN = 3
SCK_PIN = 2

CH1_PIN = 26
CH2_PIN = 19
CH3_PIN = 13
CH4_PIN = 6

c1 = False
c2 = False
c3 = False
c4 = False
rsk = False
ser = False
sck = False

GPIO.setmode(GPIO.BCM)
GPIO.setup(SER_PIN, GPIO.OUT)
GPIO.setup(SCK_PIN, GPIO.OUT)
GPIO.setup(RSK_PIN, GPIO.OUT)
GPIO.setup(CH1_PIN, GPIO.OUT)
GPIO.setup(CH2_PIN, GPIO.OUT)
GPIO.setup(CH3_PIN, GPIO.OUT)
GPIO.setup(CH4_PIN, GPIO.OUT)

GPIO.output(CH1_PIN, 0)
GPIO.output(CH2_PIN, 0)
GPIO.output(CH3_PIN, 0)
GPIO.output(CH4_PIN, 0)

def printState():
    print("1\t2\t3\t4\tRSK\tSER\tSCK")
    print(c1, c2, c3 ,c4, rsk, ser, sck, sep="\t")

while True:
    printState()
    command = input()
    if command == "1":
        c1 = not c1
        GPIO.output(CH1_PIN, c1)
    elif command == "2":
        c2 = not c2
        GPIO.output(CH2_PIN, c2)
    elif command == "3":
        c3 = not c3
        GPIO.output(CH3_PIN, c3)
    elif command == "4":
        c4 = not c4
        GPIO.output(CH4_PIN, c4)
    elif command == "r":
        rsk = not rsk
        GPIO.output(RSK_PIN, rsk)
    elif command == "ser":
        ser = not ser
        GPIO.output(SER_PIN, ser)
    elif command == "sck":
        sck = not sck
        GPIO.output(SCK_PIN, sck)