def add_numbers(a=1, b=2):
    print("this is a function in python to add two numbers:")
    result = a + b
    print(a, " + ", b, " = ", result)
    return result


if __name__ == "__main__":
    print("Running functions.py as __main__")
    a = 10
    b = 34
    c = add_numbers(a, b)
    print("Result: {}".format(c))