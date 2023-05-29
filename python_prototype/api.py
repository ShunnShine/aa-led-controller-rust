import flask
from flask import request, jsonify
from flask_cors import CORS

from exceptions.InvalidAPIUseage import InvalidAPIUsage
from exceptions.InvalidLED import InvalidLED
from led_controller.LedController import LedController

app = flask.Flask(__name__)
CORS(app)

led_controller = LedController(8, 16)
led_controller.set_all_off()

@app.errorhandler(InvalidAPIUsage)
@app.errorhandler(InvalidLED)
def invalid_api_usage(e):
    return jsonify(e.to_dict()), e.get_status_code()

@app.route('/api/led_control', methods=['POST'])
def led_control():
    body = request.json

    if 'leds_on' in body:
        leds_on = body['leds_on']
        led_controller.set_exclusive_on(leds_on)

    if 'leds_off' in body:
        leds_off = body['leds_off']
        led_controller.set_exclusive_off(leds_off)

    if 'all_leds_off' in body and body['all_leds_off']:
        led_controller.turn_all_off()

    if 'all_leds_on' in body and body['all_leds_on']:
        led_controller.turn_all_on()

    if 'leds_blink' in body:
        leds_off = body['leds_blink']
        led_controller.set_exclusive_off(leds_off)

    return jsonify({'message': 'Success!'}), 200

app.run(host='0.0.0.0', port=5000)