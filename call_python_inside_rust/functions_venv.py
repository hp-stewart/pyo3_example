import random    # part of pythons standard library; no extra install required
import emoji     # external library; install using "pip3 install emoji" or by running setup.sh

def random_number(a: int = 0, b: int = 100)-> int:
    random.seed()
    n: int = random.randint(a, b)
    print("Here's a random number from {} to {}:  {}".format(a,b,n) )
    return n

def emoji_test():
    print(emoji.emojize('Rust is :thumbs_up:'))
