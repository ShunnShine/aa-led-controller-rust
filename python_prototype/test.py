import time
from LedController import LedController

SER_PIN = 4
SCK_PIN = 2
RSK_PIN = 3

CHL_ENA_1_PIN = 26
CHL_ENA_2_PIN = 19
CHL_ENA_3_PIN = 13
CHL_ENA_4_PIN = 6

controller = LedController(SER_PIN, SCK_PIN, RSK_PIN, CHL_ENA_1_PIN, CHL_ENA_2_PIN, CHL_ENA_3_PIN, CHL_ENA_4_PIN)
controller.start()

toggle = True
controller.turn_all_off
# while True:
#     if toggle:
#         func = controller.turn_on
#     else:
#         func = controller.turn_off
#     for r in range(3, 32, 4):
#         func(r)
#         print(r)
#         input()
#     for r in range(34, 64, 4):
#         func(r)
#         print(r)
#         input()
#     for r in range(65, 96, 4):
#         func(r)
#         print(r)
#         input()
#     for r in range(96, 128, 4):
#         func(r)
#         print(r)
#         input()
#     toggle = not toggle
    