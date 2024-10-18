const int sensorPin = A0;
const float baselineTemp = 20.8;

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);

  for(int pinNo = 2; pinNo < 5; pinNo++) {
    pinMode(pinNo, OUTPUT);
    digitalWrite(pinNo, LOW);
  }
}

void loop() {
  // put your main code here, to run repeatedly:
  int sensorVal = analogRead(sensorPin);

  Serial.print("Sensor Value: ");
  Serial.print(sensorVal);

  //convert ADC to voltage 0-5V
  float voltage = (sensorVal / 1024.0) * 5;

  Serial.print(", Volts: ");
  Serial.print(voltage);

  float temp = (voltage - 0.5) * 100;

  Serial.print(", Degrees C: ");
  Serial.println(temp);

  if(temp < baselineTemp + 2) {
    digitalWrite(2, LOW);
    digitalWrite(3, LOW);
    digitalWrite(4, LOW);
  } else if(temp >= baselineTemp + 2 && temp < baselineTemp + 4) {
    digitalWrite(2, HIGH);
    digitalWrite(3, LOW);
    digitalWrite(4, LOW);
  } else if(temp >= baselineTemp + 4 && temp < baselineTemp + 6) {
    digitalWrite(2, HIGH);
    digitalWrite(3, HIGH);
    digitalWrite(4, LOW);
  } else if(temp >= baselineTemp + 6) {
    digitalWrite(2, HIGH);
    digitalWrite(3, HIGH);
    digitalWrite(4, HIGH);
  }

  delay(1);
}
