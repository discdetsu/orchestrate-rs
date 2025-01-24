from flask import Flask, jsonify
import os

app = Flask(__name__)

@app.route("/", methods=["POST"])
def predict():
    service_name = "service-1"
    response = {
        "status_code": 200,
        "app_code": "",
        "message": "Request Successful",
        "data": {
            "api_version": "1.0.0",
            "service_name": service_name,
            "prediction": {
                "image": "<heatmap_base64_image>",
                "json": {
                    "class_1": 0.6884385,
                    "class_2": 0.03514688,
                    "class_3": 0.008889809,
                },
            },
        },
    }
    return jsonify(response)

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=5001)