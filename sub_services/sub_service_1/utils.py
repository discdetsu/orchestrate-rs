import cv2
import io
import base64
import numpy as np

def generate_mockup_image():
    """Generate a heatmap-like image using OpenCV and convert it to base64."""
    # Create a random grayscale image (10x10)
    data = np.random.rand(100, 100) * 255
    data = data.astype(np.uint8)

    # Apply colormap to simulate a heatmap
    heatmap = cv2.applyColorMap(data, cv2.COLORMAP_JET)

    # Encode image as PNG
    _, buffer = cv2.imencode(".png", heatmap)

    # Convert image to base64
    image_base64 = base64.b64encode(buffer).decode("utf-8")

    return image_base64