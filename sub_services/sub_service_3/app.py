from flask import Flask, jsonify
import os
from utils import generate_mockup_image

app = Flask(__name__)

heatmap_base64 = generate_mockup_image()

@app.route("/", methods=["POST"])
def predict():
    service_name = "service-3"
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
                    "class_1": 0.3464385,
                    "class_2": 0.923481,
                    "class_3": 0.0043134,
                },
            },
        },
    }
    return jsonify(response)

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=5003)