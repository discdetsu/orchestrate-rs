# Use Python base image
FROM python:3.12-slim

# Set working directory
WORKDIR /app

# Copy application files
COPY . .

# Install required libraries (Flask)
RUN pip install flask numpy opencv-python-headless

# Expose port for Flask app
EXPOSE 5003

# Run the app
CMD ["python", "app.py"]