import time
import threading
import multiprocessing
import numpy as np
import RPi.GPIO as GPIO

GPIO.setmode(GPIO.BCM)

class LedController:
    bit_order = False

    def __init__(self, ser_pin, sck_pin, rsk_pin, ch1_pin, ch2_pin, ch3_pin, ch4_pin):
        self.ser_pin = ser_pin
        self.sck_pin = sck_pin
        self.rsk_pin = rsk_pin
        self.ch1_pin = ch1_pin
        self.ch2_pin = ch2_pin
        self.ch3_pin = ch3_pin
        self.ch4_pin = ch4_pin

        GPIO.setup(ser_pin, GPIO.OUT)
        GPIO.setup(sck_pin, GPIO.OUT)
        GPIO.setup(rsk_pin, GPIO.OUT)
        GPIO.setup(ch1_pin, GPIO.OUT)
        GPIO.setup(ch2_pin, GPIO.OUT)
        GPIO.setup(ch3_pin, GPIO.OUT)
        GPIO.setup(ch4_pin, GPIO.OUT)

        GPIO.output(ch1_pin, 0)
        GPIO.output(ch2_pin, 0)
        GPIO.output(ch3_pin, 0)
        GPIO.output(ch4_pin, 0)

        self.led_state = multiprocessing.Array('L', [0] * 4)

    def start(self):
        p = multiprocessing.Process(target=self.__start_led_ctrl_loop)
        p.start()

    def turn_on(self, light_num):
        (chl, offset) = self.__parse_light_num(light_num)
        self.led_state[chl] = self.led_state[chl] | (1 << offset)

    def turn_off(self, light_num):
        (chl, offset) = self.__parse_light_num(light_num)
        self.led_state[chl] = self.led_state[chl] & ~(1 << offset)

    def turn_all_on(self):
        self.led_state[0] = 2 ** 32 - 1
        self.led_state[1] = 2 ** 32 - 1
        self.led_state[2] = 2 ** 32 - 1
        self.led_state[3] = 2 ** 32 - 1

    def turn_all_off(self):
        self.led_state[0] = 0
        self.led_state[1] = 0
        self.led_state[2] = 0
        self.led_state[3] = 0

    def set_exclusive_on(self, light_nums):
        new_state = [0] * 4
        for num in light_nums:
            (chl, offset) = self.__parse_light_num(num)
            new_state[chl] = new_state[chl] | (1 << offset)

        self.led_state[0] = new_state[0]
        self.led_state[1] = new_state[1]
        self.led_state[2] = new_state[2]
        self.led_state[3] = new_state[3]

    def set_exclusive_off(self, light_nums):
        new_state = [0] * 4
        for num in light_nums:
            (chl, offset) = self.__parse_light_num(num)
            new_state[chl] = new_state[chl] & ~(1 << offset)

        self.led_state[0] = new_state[0]
        self.led_state[1] = new_state[1]
        self.led_state[2] = new_state[2]
        self.led_state[3] = new_state[3]

    def __start_led_ctrl_loop(self):
        try:
            while True:
                nxt_led_state = self.led_state

                self.__write_data(self.ch1_pin, self.ch4_pin, nxt_led_state[0])
                self.__write_data(self.ch2_pin, self.ch1_pin, nxt_led_state[1])
                self.__write_data(self.ch3_pin, self.ch2_pin, nxt_led_state[2])
                self.__write_data(self.ch4_pin, self.ch3_pin, nxt_led_state[3])

        finally:
            GPIO.cleanup()

    def __write_data(self, target_chl, prev_chl, val):
        GPIO.output(prev_chl, 0)
        self.__shiftOut_32(val)
        GPIO.output(self.rsk_pin, 0)
        GPIO.output(self.rsk_pin, 1)
        GPIO.output(target_chl, 1)

    def __shiftOut_32(self, val):
        for i in range(32):
            if self.bit_order:
                on = 0 if not (val & (1 << i)) else 1
            else:
                on = 0 if not (val & (1 << (31 - i))) else 1

            GPIO.output(self.ser_pin, on)

            GPIO.output(self.sck_pin, 1)
            GPIO.output(self.sck_pin, 0)

    def __parse_light_num(self, light_num):
        chl = light_num // 32
        offset = light_num % 32

        return (chl, offset)

        

        





