# install aws cli using curl
# use $ aws configure to enter credentials (Access Key, Secret Key, and Session Token) 

import boto3
from botocore.exceptions import BotoCoreError, ClientError
import os
import sys
from contextlib import closing
import pathlib
import subprocess

def polly_demo(text):
    print("Creating Speech from text using Polly")
    p = AmazonPolly()
    p.set_dialog(text)
    p.generate_audio()
    p.play_audio()

class AmazonPolly():
    
    def __init__(self):
        self.polly = boto3.client('polly')
        self.VOICE_ID = 'Joanna'
        self.text = None
        self.output_audio_file = os.path.join(pathlib.Path(__file__).resolve().parent, "output.mp3")
        self.output_format = 'mp3'
        

    def set_dialog(self, text):
        if not (type(text) is str):
            raise TypeError("Dialog must be a string")
            
        self.text = text    
    
    def generate_audio(self):
        # make sure input exists
        if self.text is None:
            raise ValueError("no text to generate audio from")
        try:
            # request speech synthesis from aws polly
            response = self.polly.synthesize_speech(OutputFormat=self.output_format, Text=self.text, VoiceId=self.VOICE_ID)
        except (BotoCoreError, ClientError) as error:
            print(error)
            raise error
            
        # Access the audio stream from the response
        if "AudioStream" in response:
        # Note: Closing the stream is important because the service throttles on the
        # number of parallel connections. Here we are using contextlib.closing to
        # ensure the close method of the stream object will be called automatically
        # at the end of the with statement's scope.
            with closing(response["AudioStream"]) as stream:
               try:
            # Open a file for writing the output as a binary stream
                    with open(self.output_audio_file, "wb") as file:
                       file.write(stream.read())
               except IOError as error:
              # Could not write to file, exit gracefully
                  raise error

        else:
        # The response didn't contain audio data, exit gracefully
            raise Exception("Could not stream audio response")
        
    def play_audio(self):
        # The following works on macOS and Linux. (Darwin = mac, xdg-open = linux).
        opener = "open" if sys.platform == "darwin" else "xdg-open"
        subprocess.call([opener, self.output_audio_file])




if __name__ == "__main__":
    polly_demo("Hello World.")