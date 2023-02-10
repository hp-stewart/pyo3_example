import sys
import datetime
import emoji

if __name__ == "__main__":
    print(emoji.emojize(f"Hello from a Python build script \nrun using `{sys.executable} \nRun at {datetime.datetime.now()}`!"))