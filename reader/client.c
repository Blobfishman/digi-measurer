
int req = 5; //mic REQ line goes to pin 5 through q1 (arduino high pulls request line low)
int dat = 2; //mic Data line goes to pin 2
int clk = 3; //mic Clock line goes to pin 3

byte mydata[14];

void setup()
{
  Serial.begin(19200);
  pinMode(req, OUTPUT);
  pinMode(clk, INPUT);
  pinMode(dat, INPUT);
  digitalWrite(clk, HIGH); // enable internal pull ups
  digitalWrite(dat, HIGH); // enable internal pull ups
  digitalWrite(req,LOW); // set request at high via transistor
}

void loop()
{
  digitalWrite(req, HIGH); // generate set request
  for(int i = 0; i < 13; i++ )
  {
    int k = 0;
   
    for (int j = 0; j < 4; j++)
    {
      while( digitalRead(clk) == LOW) { } // hold until clock is high
      while( digitalRead(clk) == HIGH) { } // hold until clock is low
      bitWrite(k, j, (digitalRead(dat) & 0x1)); // read data bits, and reverse order )
    }

    mydata[i] = k;
   
  }
   
  long num;
  double measurement;
  char buf[7];

  // Read mydata array into a char array buffer (buf) and pad with 0, for conversion to a long
  for(int lp=0;lp<6;lp++)
    buf[lp]=mydata[lp+5]+'0';

  // Convert buf to a long
  num = atol(buf);

  // If the measurement was negative make num negative.
  if(mydata[4]==8)
    num = -num;

  // Convert num to a value with the decimal point in place
  measurement = (double)num * (double)pow(10,-(mydata[11]));

  // Display the measurement, with correct amount of decimal places and in/mm
  Serial.print(measurement, mydata[11]);
  if(mydata[12])
    Serial.print(" in\n");
  else
    Serial.print(" mm\n");

  digitalWrite(req,LOW);
  delay(100);
}
