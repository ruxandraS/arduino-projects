const int redLEDPin = 11;
const int blueLEDPin = 10;
const int greenLEDPin = 9;

const int redSensorPin = A0;
const int greenSensorPin = A1;
const int blueSensorPin = A2;

int redValue = 0;
int greenValue = 0;
int blueValue = 0;

int redSensorValue = 0;
int greenSensorValue = 0;
int blueSensorValue = 0;

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);

  pinMode(redLEDPin, OUTPUT);
  pinMode(greenLEDPin, OUTPUT);
  pinMode(blueLEDPin, OUTPUT);
}

void loop() {
  // put your main code here, to run repeatedly:
  redSensorValue = analogRead(redSensorPin);
  delay(5); // ADC takes a ms to do its work
  greenSensorValue = analogRead(greenSensorPin);
  delay(5); // ADC takes a ms to do its work
  blueSensorValue = analogRead(blueSensorPin);

  Serial.print("Raw sensor values: \t red ");
  Serial.print(redSensorValue);
  Serial.print("\t green ");
  Serial.print(greenSensorValue);
  Serial.print("\t blue ");
  Serial.println(blueSensorValue);

  redValue = redSensorValue / 4;
  greenValue = greenSensorValue / 4;
  blueValue = blueSensorValue / 4;

  Serial.print("Mapped sensor values: \t red ");
  Serial.print(redValue);
  Serial.print("\t green ");
  Serial.print(greenValue);
  Serial.print("\t blue ");
  Serial.println(blueValue);

  analogWrite(redLEDPin, redValue);
  analogWrite(greenLEDPin, greenValue);
  analogWrite(blueLEDPin, blueValue);
}
