#!/usr/bin/python3

"""
Make sure you have built the project, renamed `libCheeseShop.so` to
`CheeseShop.so`, and have that file in the same directory as this .py file.

Note that /// comments in Rust will translate to Python help() documentation:
>>> help(CheeseShop.do_something)
Help on built-in function do_something:

do_something(...)
    Does something completely different by returning a Python `List[str]`.
"""

from CheeseShop import *



# MODULE-LEVEL FUNCTIONS, not particularly interesting
assert do_something() == ['And', 'now', 'for', 'something', 'completely', 'different']

for movie, year in movies():
    print("%s was released in %d" % (movie, year))


## CHEESE SHOP
# https://montycasinos.com/montypython/scripts/cheese.php.html
# How many cheeses do we not have?
assert CheeseShop.unavailable_cheese_count() == 45

cs = CheeseShop(is_hungry=True)
assert str(cs) == 'CheeseShop(stock=0)'

cs.camelot = "silly" # prints 'Setting self.camelot = "silly"'
try:
   foo = cs.does_not_exist
except ValueError as e:
    print(f'Got expected ValueError: {e}') # "Attribute not found. Move along."

# What kinds of chesee do we have?
assert not cs.has_cheese("cheddar") # prints "We have no cheddar"
assert not cs.has_cheese("greek feta") # prints "We have no greek feta"
assert not cs.has_cheese() # prints "No cheese whatsoever."

# Run the front of the shop
print(cs.respond_to_client("camembert")) # "Oh! The cat's eaten it.cheddar"
print(cs.respond_to_client("some cheese whose name I just made up")) # "No"


## ARGUMENT
# https://montycasinos.com/montypython/scripts/argument.php.html
# Let's have an argument. Default bool values
assert are_we_arguing() == '''Yes, but it isn't just saying "No, it isn't."'''
assert are_we_arguing(False) == '''Yes, but it isn't just saying "No, it isn't."'''
assert are_we_arguing(True) == "If I argue with you, I must take up a contrary position!"
# And default string values
assert ive_told_you_once() == "Yes I have."
assert ive_told_you_once("No you haven't.") == "Yes I have."
assert ive_told_you_once("When?") == "Just now."

# Keyword arguments
# Arthur has several knights: https://montycasinos.com/montypython/grailmm1.php.html#Scene%206
print("No knights at Camelot:")
knights_at_camelot()

print("Arthur's closest knights:")
knights_at_camelot(Bedevere='Wise', Lancelot='Brave', Galahad='Pure', Robin='Not Quite so Brave as Sir Lancelot') 

# `*args` for things that float
# https://montycasinos.com/montypython/grailmm1.php.html#Scene%205
things_that_float("Bread", "Apples", "Very small rocks", "Cider")
things_that_float("Great gravy", "Churches", "Lead")
things_that_float("A duck")

# Passing a callable object (that accepts no arguments!)
# https://montycasinos.com/montypython/scripts/spamskit.php.html
make_the_call(lambda: "Bloody vikings") # prints "Got a string value: 'Bloody vikings'"
make_the_call(lambda: 42) # prints "Got an integral value: 42"
make_the_call(lambda: 1.618) # prints "Got a real value: 1.618"

# Passing a callable object that expects arguments
# Some things are unexpected: https://montycasinos.com/montypython/scripts/spanish.php.html
def product_is_even(i, j):
   if i == 4 and j == 5: return "The Spanish Inquisition!" # this will raise a ValueError and stop further processing within call_with_args
   return (i * j) % 2 == 0

def pow_less_than_20(i, j):
   return (i ** j) < 20

try:
    print("Calling 'product_is_even(5, 6)':")
    call_with_args(product_is_even, 5, 6)
except ValueError as e:
    print(f'Aborting due to ValueError: {e}')

print("Calling 'pow_less_than_20(5, 6)':")
call_with_args(pow_less_than_20, 5, 6)


## DEFENSE AGAINST FRESH FRUIT
# https://montycasinos.com/montypython/scripts/fruit.php.html
# Code in self_defense.rs is intended to show how objects are passed between Rust and Python,
# how to implement Python dunder methods, documentation, etc.
# The actual Python code is very straightforward. First, we need a self-defense instructor
instructor = Instructor()

# And students, each of whom is wielding some type of weapon
harrison = Student('banana')
thompson = Student('raspberry')
student3 = Student('basket of raspberries')
student4 = Student('pointed stick')

# We define Student.attack(), but we'll instead invoke Instructor.defend(Student)
# This will first print "Student attacks with a banana"
# Then it prints "Instructor shoots Mr Apricot"
instructor.defend(harrison)
