# standard library imports; no install or venv activation required
import random    
# external library; install using "pip3 install <name>" or by running setup.sh which installs everything listed in requirements.txt
import emoji     
import optional 

def random_number(a: int = 0, b: int = 100)-> int:
    random.seed()
    n: int = random.randint(a, b)
    print("Here's a random number from {} to {}:  {}".format(a,b,n) )
    return n

def emoji_test():
    print(emoji.emojize('Rust is :thumbs_up:'))

# This function is used only by example 7c
# The when this function is called in Rust, the final output will be in the form Result<Option<char>, Error>
def color_emoji(color:str):
    if not color.isalpha():
        raise ValueError("No numbers allowed in color name")
    # format string into color code
    emoji_string = ":" + color.lower() + "_circle:"
    print("\nAttempting to display Emoji for code: " + emoji_string)

    try:
        # attempt to create an emoji from color code
        output_emoji = emoji.emojize(emoji_string)

        # if the code has an associated emoji, display and return it inside Option()
        # if no matching emoji is found, return None inside Option
        if emoji.is_emoji(output_emoji):
            # display the emoji
            print(emoji.emojize(output_emoji, language='alias'))
            # return pythons version of Option Some(emoji)
            return optional.Optional.of(output_emoji)
        else:
            print("No emoji with this name exists") 
            # return pythons version of Option None
            return optional.Optional.empty()

    except:
        raise Exception("Error occured, could not create emoji")

if __name__ == "__main__":
    emoji_test()
    color_emoji("red")
    color_emoji("blue")
    color_emoji("cyan")