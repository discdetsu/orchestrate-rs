from flask import Flask, jsonify
import os
from utils import generate_mockup_image

app = Flask(__name__)

heatmap_base64 = generate_mockup_image()

@app.route("/", methods=["POST"])
def predict():
    service_name = "service-2"
    response = {
        "status_code": 200,
        "app_code": "",
        "message": "Request Successful",
        "data": {
            "api_version": "1.0.0",
            "service_name": service_name,
            "prediction": {
                "image": heatmap_base64,
                "json": {
                    "class_1": 0.9999999,
                    "class_2": 0.7777777,
                    "class_3": 0.5555555,
                },
            },
        },
    }
    return jsonify(response)

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=5002)