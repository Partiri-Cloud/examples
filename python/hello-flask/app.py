import os

from flask import Flask, jsonify

app = Flask(__name__)

PORT = int(os.environ.get('PORT', 3000))


@app.get('/')
def index():
    return jsonify({'message': 'Hello from Flask on Partiri!'})


@app.get('/health')
def health():
    return jsonify({'status': 'ok'})


if __name__ == '__main__':
    app.run(host='0.0.0.0', port=PORT)
